use std::sync::Arc;
use dotenvy;
use tokio::net::TcpListener;

mod config;
mod state;
mod database;
mod router;
mod fronend;
use database::{
    postgres::{ 
        postgress_connect,
        postgres_migration 
    },
    redis::redis_connect
};
use state::AppState;
use router::router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::from_env();
    let postgres_pool = postgress_connect(config.clone())
        .await
        .unwrap();
    postgres_migration(&postgres_pool)
        .await;
    let connect_redis = redis_connect(config.clone())
        .await;
    let app_state = Arc::new(AppState {
        config: config.clone(),
        postgres: postgres_pool,
        redis: connect_redis,
    }); 
    let url = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(url)
        .await
        .unwrap();
    println!("Listening: {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, router(app_state)).await;
}
