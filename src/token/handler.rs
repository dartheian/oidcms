use super::extractor::TokenParams;
use super::jwt;
use crate::bounded_string::SecureString;
use crate::data::access_token::AccessToken;
use crate::data::pkce::{CodeChallenge, CodeVerifier};
use crate::data::time::UnixTime;
use crate::data::{pkce, AuthenticationMethod, IdToken, Scope, TokenType};
use crate::state::{AppState, AuthSession};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response, Result};
use axum::Json;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: u64,
    id_token: String,
    scope: HashSet<Scope>,
    token_type: TokenType,
}

#[derive(Debug, Error)]
pub enum InvalidParamError {
    #[error("`client_secret` does not match: expected `{0}` got `{0}`")]
    ClientSecret(SecureString, SecureString),
    #[error("no auth session associated with code `{0}`")]
    Code(SecureString),
    #[error("pkce verification failed: expected `{0}` got `{1}`")]
    Grant(CodeChallenge, CodeVerifier),
    #[error("`redirect_uri` does not match: expected `{0}` got `{1}`")]
    RedirectUri(Uri, Uri),
    #[error("jwt encode error: `{0}`")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for InvalidParamError {
    fn into_response(self) -> Response {
        let response = match self {
            Self::Code(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Grant { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        response.into_response()
    }
}

pub async fn token(state: AppState, params: TokenParams) -> Result<impl IntoResponse> {
    let auth_session = get_session(&state, params.code)?;
    verify_pkce(auth_session.code_challenge, params.code_verifier)?;
    verify_redirect_uri(auth_session.redirect_uri, params.redirect_uri)?;
    verify_client_secret(params.client_secret, state.client_secret())?;
    let now = UnixTime::now();
    let access_token = AccessToken {
        aud: state.audience(),
        auth_time: now,
        cid: auth_session.client_id.clone(),
        exp: now + state.expiration(),
        iat: now,
        iss: state.issuer(),
        jti: state.gen_secure_string(),
        scp: auth_session.scope.clone(),
        sub: auth_session.user_id.clone(),
        uid: auth_session.user_id.clone(),
        ver: 1,
    };
    let access_token = jwt::encode(access_token, state.secret()).unwrap();
    let id_token = IdToken {
        amr: vec![AuthenticationMethod::Pwd],
        at_hash: access_token_hash(&access_token),
        aud: auth_session.client_id,
        auth_time: now,
        exp: now + state.expiration(),
        iat: now,
        iss: state.issuer(),
        jti: state.gen_secure_string(),
        sub: auth_session.user_id,
        ver: 1,
    };
    Ok(Json(TokenResponse {
        access_token,
        expires_in: state.expiration(),
        id_token: jwt::encode(id_token, state.secret()).unwrap(),
        scope: auth_session.scope,
        token_type: TokenType::Bearer,
    }))
}

fn get_session(state: &AppState, code: SecureString) -> Result<AuthSession, InvalidParamError> {
    state
        .get_session(&code)
        .ok_or(InvalidParamError::Code(code))
}

fn verify_pkce(challenge: CodeChallenge, verifier: CodeVerifier) -> Result<(), InvalidParamError> {
    if pkce::verify(&challenge, &verifier) {
        Ok(())
    } else {
        Err(InvalidParamError::Grant(challenge, verifier))
    }
}

fn verify_redirect_uri(expected: Uri, got: Uri) -> Result<(), InvalidParamError> {
    if expected == got {
        Ok(())
    } else {
        Err(InvalidParamError::RedirectUri(expected, got))
    }
}

fn verify_client_secret(
    expected: SecureString,
    got: SecureString,
) -> Result<(), InvalidParamError> {
    if expected == got {
        Ok(())
    } else {
        Err(InvalidParamError::ClientSecret(expected, got))
    }
}

fn access_token_hash(access_token: &String) -> String {
    let hash = Sha256::digest(access_token);
    let half_length = hash.len() / 2;
    let half_hash = &hash[..half_length];
    BASE64_URL_SAFE_NO_PAD.encode(half_hash)
}
