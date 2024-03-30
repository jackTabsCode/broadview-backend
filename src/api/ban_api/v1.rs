use crate::{
    database::Database,
    guards::api_key::ApiKey,
    models::{Ban, V1BanResult},
};
use rocket::{serde::json::Json, State};

#[get("/ban/<user_id>")]
pub async fn get_ban(_api_key: ApiKey, db: &State<Database>, user_id: &str) -> Json<V1BanResult> {
    let user_id = user_id.parse::<u64>().unwrap();

    match db.find_active_ban(user_id).await {
        Some(ban) => Json(V1BanResult::Banned {
            banned: serde_bool::True,
            ban,
        }),
        None => Json(V1BanResult::NotBanned {
            banned: serde_bool::False,
        }),
    }
}

#[get("/bans")]
pub async fn get_all_bans(db: &State<Database>) -> Json<Vec<Ban>> {
    db.get_all_bans().await.into()
}

#[put("/ban", data = "<ban>")]
pub async fn put_ban(_api_key: ApiKey, db: &State<Database>, ban: Json<Ban>) {
    db.insert_ban(ban.into_inner()).await.unwrap();
}

#[delete("/ban/<user_id>")]
pub async fn delete_ban(_api_key: ApiKey, db: &State<Database>, user_id: &str) {
    let user_id = user_id.parse::<u64>().unwrap();
    db.remove_ban(user_id).await;
}
