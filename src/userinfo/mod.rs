use crate::data::access_token::{self, AccessToken};
use crate::state::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Result};
use axum::Json;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid access_token: {0}")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("invalid access_token claim: {0}")]
    InvalidClaim(#[from] access_token::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response = match self {
            Self::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::InvalidClaim(_) => (StatusCode::FORBIDDEN, self.to_string()),
        };
        response.into_response()
    }
}

pub async fn userinfo(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: AppState,
) -> Result<impl IntoResponse> {
    let access_token = decode_access_token(&state, bearer)?;
    access_token.validate(&state).map_err(Error::InvalidClaim)?;
    let user_info = state.get_user();
    Ok(Json(user_info))
}

fn decode_access_token(state: &AppState, bearer: Bearer) -> Result<AccessToken, Error> {
    let key = DecodingKey::from_secret(&state.secret().as_ref());
    let validation = Validation::default();
    let a = decode(bearer.token(), &key, &validation)?;
    Ok(a.claims)
}
