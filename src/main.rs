use std::env;

use axum::{routing::get, serve, Router};
use database::Database;
use dotenv::dotenv;
use roboat::ClientBuilder;

mod database;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = Database::new().await;

    // let roblosecurity = env::var("ROBLOX_COOKIE").expect("ROBLOX_COOKIE must be set");
    // let roboat_client = ClientBuilder::new().roblosecurity(roblosecurity).build();

    let app = Router::new().route("/", get(|| async { "Hello world!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    serve(listener, app).await.unwrap()
}
