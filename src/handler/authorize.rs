use crate::extractor::authorize::AuthorizeParams;
use crate::parameter::client_id::ClientId;
use crate::parameter::code::Code;
use crate::parameter::pkce::CodeChallenge;
use crate::parameter::scope::ScopeSet;
use crate::parameter::state::State;
use askama::Template;
use axum::http::{HeaderMap, Uri};
use headers::{CacheControl, ContentType, HeaderMapExt, Pragma};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthSession {
    pub client_id: ClientId,
    pub code_challenge: CodeChallenge,
    #[serde(with = "http_serde::uri")]
    pub redirect_uri: Uri,
    pub scope: ScopeSet,
}

impl From<&AuthorizeParams> for AuthSession {
    fn from(value: &AuthorizeParams) -> Self {
        Self {
            client_id: value.client_id.clone(),
            code_challenge: value.code_challenge.clone(),
            redirect_uri: value.redirect_uri.clone(),
            scope: value.scope.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct AuthorizeResponse {
    code: Code,
    redirect_uri: Uri,
    state: State,
}

pub async fn authorize(
    session: Session,
    params: AuthorizeParams,
) -> (HeaderMap, AuthorizeResponse) {
    let code = Code::new();
    let login_session = AuthSession::from(&params);
    session.insert(code.as_ref(), login_session).await.unwrap();
    let mut headers = HeaderMap::new();
    headers.typed_insert(ContentType::html());
    headers.typed_insert(CacheControl::new().with_no_cache().with_no_store());
    headers.typed_insert(Pragma::no_cache());
    (
        headers,
        AuthorizeResponse {
            code: code,
            redirect_uri: params.redirect_uri,
            state: params.state,
        },
    )
}
