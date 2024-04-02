use crate::{auth::ApiKey, state::AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

const GROUP_ID: u64 = 3016035;
const ROLE_ID: u64 = 20676927;

pub async fn put_resident(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    ApiKey: ApiKey,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = user_id.parse::<u64>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Failed to parse user ID".to_string(),
        )
    })?;

    let attempt = state
        .roboat_client
        .set_group_member_role(user_id, GROUP_ID, ROLE_ID)
        .await;

    match attempt {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
