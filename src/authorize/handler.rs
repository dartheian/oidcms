use super::extractor::AuthorizeParams;
use crate::parameter::State;
use crate::session::{AuthSession, Session};
use crate::{parameter::code::Code, session};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State as AxumState;
use axum::http::{HeaderMap, Uri};
use headers::{CacheControl, ContentType, HeaderMapExt, Pragma};

impl From<AuthorizeParams> for AuthSession {
    fn from(value: AuthorizeParams) -> Self {
        Self {
            client_id: value.client_id,
            code_challenge: value.code_challenge,
            redirect_uri: value.redirect_uri,
            scope: value.scope,
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

pub async fn authorize(session: AxumState<Session>, params: AuthorizeParams) -> impl IntoResponse {
    let code = Code::new();
    session::set(&session, code.clone(), params.clone().into());
    let mut headers = HeaderMap::new();
    headers.typed_insert(ContentType::html());
    headers.typed_insert(CacheControl::new().with_no_cache().with_no_store());
    headers.typed_insert(Pragma::no_cache());
    (
        headers,
        AuthorizeResponse {
            code,
            redirect_uri: params.redirect_uri,
            state: params.state,
        },
    )
}
