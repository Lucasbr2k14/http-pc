use axum::{
    Router, 
    http::StatusCode, 
    response::{ Html, Json },
    routing::{ get, post },
    extract::Path
};

use dotenvy;
use tokio::net::TcpListener;
use askama::Template;

mod config;
mod state;
mod database;

#[tokio::main]
async fn main() {
    // Start .env
    dotenvy::dotenv().ok();

    let config = config::from_env();

    // Criar o servidor
    let url = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(url).await.unwrap();

    println!("Listening: {}", listener.local_addr().unwrap());

    // Criando as rotas
    let _ = axum::serve(listener, router()).await;
}


fn router() -> Router {
    let router = Router::new()
    .route("/", get(root))
    .route(
        "/json/{id}", 
        get( move |path| test_json(path) )
    );

    router
}

#[derive(Template)]
#[template(path = "index.html.jinja")]
struct Index;

async fn root() -> Html<String> {
    let template = Index.render().unwrap();
    Html(template)
}

async fn test_json( Path(user_id):Path<String> ) -> Json <serde_json::Value> {
    let res = serde_json::json!(
        {
            "Ok":user_id
        }
    );

    Json(res)
}