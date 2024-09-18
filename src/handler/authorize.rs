use crate::extractor::AuthorizeParams;
use crate::parameters::client_id::ClientId;
use crate::parameters::pkce::CodeChallenge;
use crate::parameters::state::State;
use askama::Template;
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use headers::{CacheControl, ContentType, HeaderMapExt, Pragma};
use serde::Serialize;
use thiserror::Error;
use tower_sessions::Session;

#[derive(Debug, Serialize)]
pub struct AuthSession {
    client_id: ClientId,
    code_challenge: CodeChallenge,
    #[serde(with = "http_serde::uri")]
    redirect_uri: Uri,
}

impl From<&AuthorizeParams> for AuthSession {
    fn from(value: &AuthorizeParams) -> Self {
        Self {
            client_id: value.client_id.clone(),
            code_challenge: value.code_challenge.clone(),
            redirect_uri: value.redirect_uri.clone(),
        }
    }
}
#[derive(Template)]
#[template(path = "form.html")]
pub struct AuthorizeResponse {
    redirect_uri: Uri,
    state: State,
}

impl From<AuthorizeParams> for AuthorizeResponse {
    fn from(value: AuthorizeParams) -> Self {
        Self {
            redirect_uri: value.redirect_uri,
            state: value.state,
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error while trying to update the session: {0}")]
    SessionError(#[from] tower_sessions::session::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

pub async fn authorize(
    session: Session,
    params: AuthorizeParams,
) -> Result<(HeaderMap, AuthorizeResponse), Error> {
    let login_session = AuthSession::from(&params);
    session.insert(params.state.as_ref(), login_session).await?;
    let mut headers = HeaderMap::new();
    headers.typed_insert(ContentType::html());
    headers.typed_insert(CacheControl::new().with_no_cache().with_no_store());
    headers.typed_insert(Pragma::no_cache());
    Ok((headers, params.into()))
}
