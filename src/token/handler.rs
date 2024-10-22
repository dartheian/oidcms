use super::extractor::TokenParams;
use super::jwt;
use crate::parameter::code::Code;
use crate::parameter::pkce::{CodeChallenge, CodeVerifier};
use crate::parameter::subject::Subject;
use crate::parameter::time::{Expiration, IssuedAt};
use crate::parameter::{pkce, AccessToken, IdToken, Scope, TokenType};
use crate::state::{AppState, AuthSession};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response, Result};
use axum::Json;
use serde::Serialize;
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
    #[error("no auth session associated with code `{0}`")]
    Code(Code),
    #[error("pkce verification failed: expected `{0}` got `{1}`")]
    Grant(CodeChallenge, CodeVerifier),
    #[error("`redirect_uri` does not match: expected `{expected}` got `{got}`")]
    RedirectUri { expected: Uri, got: Uri },
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
    let subject: Subject = state.gen_random();
    let access_token = AccessToken {
        aud: auth_session.client_id.clone(),
        exp: Expiration::new(state.expiration()),
        iat: IssuedAt::new(),
        iss: state.issuer(),
        sub: subject.clone(),
    };
    let id_token = IdToken {
        client_id: auth_session.client_id,
        exp: Expiration::new(state.expiration()),
        iat: IssuedAt::new(),
        iss: state.issuer(),
        sub: subject,
    };
    Ok(Json(TokenResponse {
        access_token: jwt::encode(access_token, state.secret()).unwrap(),
        expires_in: state.expiration(),
        id_token: jwt::encode(id_token, state.secret()).unwrap(),
        scope: auth_session.scope,
        token_type: TokenType::Bearer,
    }))
}

fn get_session(state: &AppState, code: Code) -> Result<AuthSession, InvalidParamError> {
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
        Err(InvalidParamError::RedirectUri { expected, got })
    }
}
