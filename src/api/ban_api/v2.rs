use crate::{database::Database, guards::api_key::ApiKey, models::Ban};
use rocket::{serde::json::Json, State};

use super::v1;

#[get("/ban/<user_id>")]
pub async fn get_ban(_api_key: ApiKey, db: &State<Database>, user_id: &str) -> Json<Option<Ban>> {
    let user_id = user_id.parse::<u64>().unwrap();

    db.find_active_ban(user_id).await.into()
}

#[get("/bans")]
pub async fn get_all_bans(db: &State<Database>) -> Json<Vec<Ban>> {
    v1::get_all_bans(db).await
}

#[put("/ban", data = "<ban>")]
pub async fn put_ban(_api_key: ApiKey, db: &State<Database>, ban: Json<Ban>) {
    v1::put_ban(_api_key, db, ban).await;
}

#[delete("/ban/<user_id>")]
pub async fn delete_ban(_api_key: ApiKey, db: &State<Database>, user_id: &str) {
    v1::delete_ban(_api_key, db, user_id).await;
}
