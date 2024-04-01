use crate::{auth::ApiKey, state::AppState};
use axum::extract::{Path, State};
use std::sync::Arc;

const GROUP_ID: u64 = 3016035;
const ROLE_ID: u64 = 20676927;

pub async fn put_resident(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
    ApiKey(): ApiKey,
) {
    let user_id = user_id.parse::<u64>().unwrap();

    state
        .roboat_client
        .set_group_member_role(user_id, GROUP_ID, ROLE_ID)
        .await
        .unwrap();
}
