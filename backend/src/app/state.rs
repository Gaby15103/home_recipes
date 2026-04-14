use sea_orm::DatabaseConnection;
use redis::Client;
use std::sync::Arc;
use sqlx::SqlitePool;
use crate::config::Config;
use crate::utils::notification_hub::NotificationHub;

pub struct AppState {
    pub db: DatabaseConnection,
    pub dict_db: SqlitePool,
    pub redis: Arc<Client>,
    pub config: Arc<Config>,
    pub notification_hub: Arc<NotificationHub>,
}
