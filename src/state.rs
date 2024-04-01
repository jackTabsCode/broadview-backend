use roboat::Client;

use crate::database::Database;

pub struct AppState {
    pub database: Database,
    pub roboat_client: Client,
}
