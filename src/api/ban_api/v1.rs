use crate::{
    auth::ApiKey,
    models::{Ban, BanRequest, V1BanResult},
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
) -> Json<V1BanResult> {
    let user_id = user_id.parse::<u64>().unwrap();

    match state.database.find_active_ban(user_id).await {
        Some(ban) => Json(V1BanResult::Banned {
            banned: serde_bool::True,
            ban,
        }),
        None => Json(V1BanResult::NotBanned {
            banned: serde_bool::False,
        }),
    }
}

pub async fn put_ban(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    ApiKey(): ApiKey,
    Json(ban): Json<BanRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = user_id.parse::<u64>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Failed to parse user ID".to_string(),
        )
    })?;

    state
        .database
        .insert_ban(Ban::from_request(ban, user_id))
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
