// async fn authorize(
//     State(state): State<Arc<Mutex<AppState>>>,
//     Query(params): Query<AuthorizeParams>,
// ) -> Result<(HeaderMap, AuthorizeResponse), AuthorizeError> {
//     params.validate()?;
//     let mut lock = state.lock().unwrap();
//     lock.client_id = Some(params.client_id);
//     lock.code_challenge = Some(params.code_challenge);
//     drop(lock);
//     let mut headers = HeaderMap::new();
//     headers.typed_insert(ContentType::html());
//     headers.typed_insert(CacheControl::new().with_no_cache().with_no_store());
//     headers.typed_insert(Pragma::no_cache());
//     Ok((
//         headers,
//         AuthorizeResponse {
//             state: params.state,
//             redirect_uri: params.redirect_uri,
//         },
//     ))
// }
