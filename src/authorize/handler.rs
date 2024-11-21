use super::extractor::AuthorizeParams;
use crate::bounded_string::SecureString;
use crate::state::{AppState, AuthSession};
use askama::Template;
use askama_axum::IntoResponse;
use axum::http::{HeaderMap, Uri};
use headers::{CacheControl, ContentType, HeaderMapExt, Pragma};

#[derive(Template)]
#[template(path = "form.html")]
pub struct AuthorizeResponse {
    code: SecureString,
    redirect_uri: Uri,
    state: SecureString,
}

pub async fn authorize(state: AppState, params: AuthorizeParams) -> impl IntoResponse {
    let session = AuthSession {
        client_id: params.client_id,
        code_challenge: params.code_challenge,
        redirect_uri: params.redirect_uri.clone(),
        scope: params.scope,
        user_id: state.gen_secure_string(),
    };
    let code: SecureString = state.gen_secure_string();
    state.set_session(code.clone(), session);
    let mut headers = HeaderMap::new();
    headers.typed_insert(ContentType::html());
    headers.typed_insert(CacheControl::new().with_no_cache().with_no_store());
    headers.typed_insert(Pragma::no_cache());
    let response = AuthorizeResponse {
        code,
        redirect_uri: params.redirect_uri,
        state: params.state,
    };
    (headers, response)
}
