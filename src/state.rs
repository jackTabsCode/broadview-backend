use roboat::Client;

use crate::database::Database;

#[derive(Clone)]
pub struct ApiKeyState(pub String);

pub struct AppState {
    pub database: Database,
    pub roboat_client: Client,

    pub api_key: ApiKeyState,
}
