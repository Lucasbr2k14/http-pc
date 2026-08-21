use sqlx::PgPool;
use redis::aio::MultiplexedConnection;

use crate::config::Configs;


#[derive(Clone)]
struct AppState {
    pub postgres: PgPool,
    pub redis: MultiplexedConnection,
    pub config: Configs,
}