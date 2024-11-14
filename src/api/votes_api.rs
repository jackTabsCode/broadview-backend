use crate::auth::ApiKey;
use axum::{http::StatusCode, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Votes {
    up_votes: u32,
    down_votes: u32,
}

#[derive(Debug, Deserialize)]
struct Data {
    data: Vec<Votes>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub likes: u32,
    pub dislikes: u32,
}

pub async fn get_votes(ApiKey: ApiKey) -> Result<Json<Response>, (StatusCode, String)> {
    let client = Client::new();

    let res = client
        .get("https://games.roblox.com/v1/games/votes?universeIds=226532341")
        .send()
        .await
        .expect("Failed to send request");

    let arr: Data = res.json().await.expect("Failed to parse JSON");
    let votes = arr.data.first().expect("No votes found");

    Ok(Json(Response {
        likes: votes.up_votes,
        dislikes: votes.down_votes,
    }))
}
