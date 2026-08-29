use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Configs;

const MAX_CONNECTIONS: u32 = 15;

pub async fn postgress_connect(conf:Configs) -> Result<PgPool, sqlx::Error> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}", 
        conf.postgres_user,
        conf.postgres_pass,
        conf.postgres_addr,
        conf.postgres_port,
        conf.postgres_data
    );

    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&url)
        .await;
    
    pool
}