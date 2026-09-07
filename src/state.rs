use sqlx::PgPool;
use redis::aio::ConnectionManager;

use crate::config::Configs;

#[derive(Clone)]
pub struct AppState {
    pub config:   Configs,
    pub redis:    ConnectionManager,
    pub postgres: PgPool,
}