use axum::{
    Router, 
    http::StatusCode, 
    response::{ Html, Json },
    routing::{ get, post },
    extract::Path
};

use std::sync::Arc;

use dotenvy;
use tokio::net::TcpListener;

mod config;
mod state;
mod database;
mod fronend;

use database::postgres::postgress_connect;
use state::AppState;


#[tokio::main]
async fn main() {
    // Start .env
    dotenvy::dotenv().ok();

    let config = config::from_env();
    
    let postgres_pool = postgress_connect(config.clone())
    .await
    .unwrap();

    // Cria o app state para passar para todas as rotas.
    let app_state = Arc::new(AppState {
        config: config.clone(),
        postgres: postgres_pool.clone(),
    
    }); 


    // Criar o servidor
    let url = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(url).await.unwrap();

    println!("Listening: {}", listener.local_addr().unwrap());

    // Criando as rotas
    let _ = axum::serve(listener, router(app_state)).await;
}

fn router( state: Arc<AppState> ) -> Router {
    let router = Router::new()
        .route(
            "/json/{id}", 
            get( move |path| test_json(path) )
        )
        .merge(fronend::routes::routes())
        .with_state(state);
    
    router
}

async fn test_json( Path(user_id):Path<String> ) -> Json <serde_json::Value> {
    let res = serde_json::json!(
        {
            "Ok":user_id
        }
    );

    Json(res)
}