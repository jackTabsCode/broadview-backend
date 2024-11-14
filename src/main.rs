use api::{ban_api, resident_api, votes_api};
use axum::{
    routing::{get, put},
    Router,
};
use database::Database;
use dotenv::dotenv;
use roboat::ClientBuilder;
use state::AppState;
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
pub mod auth;
mod database;
mod models;
pub mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let roblox_cookie = get_roblox_cookie().expect("failed to get Roblox cookie");
    let api_key = env::var("BACKEND_API_KEY").expect("BACKEND_API_KEY must be set");

    let database = Database::new().await;
    let roboat_client = ClientBuilder::new().roblosecurity(roblox_cookie).build();

    let state = Arc::new(AppState {
        database,
        roboat_client,
        api_key: state::ApiKeyState(api_key),
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/v1/ban/:user_id",
            get(ban_api::v1::get_ban)
                .delete(ban_api::v1::delete_ban)
                .put(ban_api::v1::put_ban),
        )
        .route("/v1/resident/:user_id", put(resident_api::v1::put_resident))
        .route("/votes", get(votes_api::get_votes))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(trace::TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::info!("Listening on {}", addr);
}

fn get_roblox_cookie() -> Result<String, String> {
    rbx_cookie::get().ok_or_else(|| {
        env::var("ROBLOX_COOKIE").unwrap_or_else(|_| "ROBLOX_COOKIE must be set".to_string())
    })
}
