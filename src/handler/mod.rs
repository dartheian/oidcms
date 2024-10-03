pub mod authorize;
pub mod token;

// async fn userinfo(
//     TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
// ) -> impl IntoResponse {
//     let access_token = decode::<AccessTokenPayload>(
//         &bearer.token(),
//         &DecodingKey::from_secret("secret".as_ref()),
//         &Validation::new(Algorithm::HS256),
//     );
//     match access_token {
//         Ok(_) => Ok(()),
//         Err(error) => Err(error.to_string()),
//     }
// }
