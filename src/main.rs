use api::{ban_api, resident_api};
use axum::{
    routing::{get, put},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use database::Database;
use dotenv::dotenv;
use roboat::ClientBuilder;
use state::AppState;
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
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

    let roblox_cookie = env::var("ROBLOX_COOKIE").expect("ROBLOX_COOKIE must be set");
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
            get(ban_api::v1::get_ban).delete(ban_api::v1::delete_ban),
        )
        .route("/v1/ban", put(ban_api::v1::put_ban))
        .route("/v1/resident", put(resident_api::v1::put_resident))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(trace::TraceLayer::new_for_http());

    let config = RustlsConfig::from_pem_file(
        PathBuf::from("broadview.crt"),
        PathBuf::from("broadview.key"),
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
