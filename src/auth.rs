use crate::state::{ApiKeyState, AppState};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use std::sync::Arc;

pub struct ApiKey;

#[async_trait]
impl<S> FromRequestParts<S> for ApiKey
where
    ApiKeyState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        let api_key = match headers.get("API-Key") {
            Some(auth) => auth,
            None => return Err((StatusCode::UNAUTHORIZED, "Missing API key")),
        };

        let api_key = api_key
            .to_str()
            .expect("authorization header should be valid ASCII");

        let state = ApiKeyState::from_ref(state);

        if api_key != state.0 {
            return Err((StatusCode::UNAUTHORIZED, "Invalid API key"));
        }

        Ok(ApiKey)
    }
}

impl FromRef<Arc<AppState>> for ApiKeyState {
    fn from_ref(state: &Arc<AppState>) -> ApiKeyState {
        state.api_key.clone()
    }
}
