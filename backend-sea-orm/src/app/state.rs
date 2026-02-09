use sea_orm::DatabaseConnection;
use redis::Client;
use std::sync::Arc;

use crate::config::Config;

pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: Arc<Client>,
    pub config: Arc<Config>,
}
