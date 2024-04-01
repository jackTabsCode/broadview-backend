use std::env;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct ApiKey();

#[async_trait]
impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let actual_api_key = env::var("BACKEND-API-KEY").expect("BACKEND-API-KEY must be set");

        let headers = parts.headers.clone();
        let api_key = match headers.get("API-Key") {
            Some(auth) => auth,
            None => return Err((StatusCode::UNAUTHORIZED, "Missing API key")),
        };

        let api_key = api_key
            .to_str()
            .expect("authorization header should be valid ASCII");

        if api_key != actual_api_key {
            return Err((StatusCode::UNAUTHORIZED, "Invalid API key"));
        }

        Ok(ApiKey())
    }
}
