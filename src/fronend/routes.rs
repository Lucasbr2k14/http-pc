use crate::state::AppState;

use std::sync::Arc;

use askama::Template;

use axum::{
    Router,
    routing::get,
};


pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root))
}