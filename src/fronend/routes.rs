use crate::state::AppState;
use super::handler::root;

use std::sync::Arc;

use axum::{
    Router,
    routing::get,
};


pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root))
}