use super::extractor::TokenParams;
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

const EXPIRES_IN: u64 = 3600;
const ISSUER: &str = "http://rain.okta1.com:1802";

#[derive(Serialize)]
pub struct TokenResponse {
    #[serde(serialize_with = "crate::serde_utils::jwt::serialize")]
    access_token: AccessToken,
    expires_in: u64,
    #[serde(serialize_with = "crate::serde_utils::jwt::serialize")]
    id_token: IdToken,
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
    let subject: Subject = state.random();
    let access_token = AccessToken {
        aud: auth_session.client_id.clone(),
        exp: Expiration::new(),
        iat: IssuedAt::new(),
        iss: Uri::try_from(ISSUER).unwrap(),
        sub: subject.clone(),
    };
    let id_token = IdToken {
        client_id: auth_session.client_id,
        exp: Expiration::new(),
        iat: IssuedAt::new(),
        iss: Uri::try_from(ISSUER).unwrap(),
        sub: subject,
    };
    Ok(Json(TokenResponse {
        access_token,
        expires_in: EXPIRES_IN,
        id_token,
        scope: auth_session.scope,
        token_type: TokenType::Bearer,
    }))
}

fn get_session(state: &AppState, code: Code) -> Result<AuthSession, InvalidParamError> {
    state.get(&code).ok_or(InvalidParamError::Code(code))
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
