use askama::Template;
use redis::AsyncCommands; 

use super::super::AppState;

use std::sync::Arc;

use axum::{
    response::Html,
    extract::State
};

// Funções de handler para cada request, do frontend.
#[derive(Template)]
#[template(path = "index.html.jinja")]
struct Index {
    counter: u32
}

pub async fn root(
    State(state): State<Arc<AppState>>
) -> Html<String> {
    let mut redis = state.redis.clone();
    let q:u32 = redis.incr("root:count", 1).await.unwrap();
    let index = Index { counter: q };
    let template = index.render().unwrap();
    Html(template)
}