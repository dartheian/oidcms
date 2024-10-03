use super::authorize::AuthSession;
use crate::parameter::access_token::{AccessToken, AccessTokenPayload};
use crate::parameter::code::Code;
use crate::parameter::id_token::{IdToken, IdTokenPayload};
use crate::parameter::pkce::{CodeChallenge, CodeVerifier};
use crate::parameter::scope::ScopeSet;
use crate::parameter::subject::Subject;
use crate::parameter::time::IssuedAt;
use crate::parameter::{pkce, TokenType};
use crate::{extractor::token::TokenParams, parameter::time::Expiration};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;
use tower_sessions::Session;

const EXPIRES_IN: u128 = 3600;
const ISSUER: &str = "http://rain.okta1.com:1802";

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: AccessToken,
    expires_in: u128,
    id_token: IdToken,
    scope: ScopeSet,
    token_type: TokenType,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("no auth session associated with the code: `{0}`")]
    InvalidCode(Code),
    #[error("the PKCE verification failed: expected `{challenge}` got `{verifier}`")]
    InvalidGrant {
        challenge: CodeChallenge,
        verifier: CodeVerifier,
    },
    #[error("the redirect URI does not match: expected `{expected}` got `{got}`")]
    InvalidRedirectUri { expected: Uri, got: Uri },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidCode(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::InvalidGrant { .. } => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
        }
    }
}

#[axum::debug_handler]
pub async fn token(session: Session, params: TokenParams) -> Result<Json<TokenResponse>, Error> {
    let Some(auth_session) = session
        .remove::<AuthSession>(params.code.as_ref())
        .await
        .unwrap()
    else {
        return Err(Error::InvalidCode(params.code));
    };
    verify_pkce(auth_session.code_challenge, params.code_verifier)?;
    verify_redirect_uri(auth_session.redirect_uri, params.redirect_uri)?;
    let subject = Subject::new();
    let access_token = AccessTokenPayload {
        aud: auth_session.client_id.clone(),
        exp: Expiration::new(),
        iat: IssuedAt::new(),
        iss: Uri::try_from(ISSUER).unwrap(),
        sub: subject.clone(),
    };
    let access_token = AccessToken::try_from(access_token).unwrap();
    let id_token = IdTokenPayload {
        client_id: auth_session.client_id,
        exp: Expiration::new(),
        iat: IssuedAt::new(),
        iss: Uri::try_from(ISSUER).unwrap(),
        sub: subject,
    };
    let id_token = IdToken::try_from(id_token).unwrap();
    Ok(Json(TokenResponse {
        access_token,
        expires_in: EXPIRES_IN,
        id_token,
        scope: auth_session.scope,
        token_type: TokenType::Bearer,
    }))
}

fn verify_pkce(challenge: CodeChallenge, verifier: CodeVerifier) -> Result<(), Error> {
    if pkce::verify(&challenge, &verifier) {
        Ok(())
    } else {
        Err(Error::InvalidGrant {
            challenge,
            verifier,
        })
    }
}

fn verify_redirect_uri(expected: Uri, got: Uri) -> Result<(), Error> {
    if expected == got {
        Ok(())
    } else {
        Err(Error::InvalidRedirectUri { expected, got })
    }
}
