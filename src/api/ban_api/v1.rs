use crate::{
    auth::ApiKey,
    models::{BanDocument, V1BanRequest, V1BanResponse},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub async fn get_ban(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    ApiKey(): ApiKey,
) -> Json<V1BanResponse> {
    let user_id = user_id.parse::<u64>().unwrap();

    match state.database.find_active_ban(user_id).await {
        Some(ban) => Json(V1BanResponse::Banned {
            banned: serde_bool::True,
            ban: ban.into(),
        }),
        None => Json(V1BanResponse::NotBanned {
            banned: serde_bool::False,
        }),
    }
}

pub async fn put_ban(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    ApiKey(): ApiKey,
    Json(ban): Json<V1BanRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = user_id.parse::<u64>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Failed to parse user ID".to_string(),
        )
    })?;

    state
        .database
        .insert_ban(BanDocument::from_request(ban, user_id))
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

pub async fn delete_ban(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    ApiKey(): ApiKey,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = user_id.parse::<u64>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Failed to parse user ID".to_string(),
        )
    })?;

    state
        .database
        .remove_ban(user_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}
