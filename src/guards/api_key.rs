use std::env;

use rocket::{
    http::Status,
    outcome::Outcome,
    request,
    request::{FromRequest, Request},
};

#[derive(Debug)]
pub struct ApiKey(pub String);

fn is_valid(key: &str) -> bool {
    key == env::var("BACKEND_API_KEY").unwrap_or("".to_string())
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("API-Key").collect();
        match keys.len() {
            0 => Outcome::Error((Status::Unauthorized, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid)),
            _ => Outcome::Error((Status::Unauthorized, ApiKeyError::BadCount)),
        }
    }
}
