use std::env;

use api::{ban_api, resident_api};
use database::Database;
use dotenv::dotenv;
use guards::api_key::ApiKey;
use roboat::ClientBuilder;
use rocket_governor::rocket_governor_catcher;

mod api;
mod database;
mod guards;
mod models;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(_api_key: ApiKey) -> &'static str {
    "Hello, world!"
}

#[catch(401)]
fn unauthorized() -> &'static str {
    "You are not authorized to access this API."
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database = Database::new().await;

    let roblosecurity = env::var("ROBLOX_COOKIE").expect("ROBLOX_COOKIE must be set");
    let roboat_client = ClientBuilder::new().roblosecurity(roblosecurity).build();

    rocket::build()
        .manage(database)
        .manage(roboat_client)
        .mount("/", routes![index])
        .mount(
            "/v1",
            routes![
                ban_api::v1::get_ban,
                ban_api::v1::get_all_bans,
                ban_api::v1::put_ban,
                ban_api::v1::delete_ban
            ],
        )
        .mount(
            "/v2",
            routes![
                ban_api::v2::get_ban,
                ban_api::v2::get_all_bans,
                ban_api::v2::put_ban,
                ban_api::v2::delete_ban,
                resident_api::make_resident
            ],
        )
        .register("/", catchers![unauthorized])
        .register("/", catchers![rocket_governor_catcher])
}
