use sqlx::PgPool;
use redis::aio::ConnectionManager;

use crate::config::Configs;


#[derive(Clone)]
pub struct AppState {
    pub postgres: PgPool,
    pub redis:    ConnectionManager,
    pub config:   Configs,
}