use std::sync::Arc;
use crate::AppState;

use axum::{
    Router,
    routing:: { get, post }
};

use super::handler;

// pub fn router_api() -> Router {}

pub fn router_web() -> Router<Arc<AppState>> {
    let router = Router::new()
        .route(
            "/register", 
            get(handler::register_web) 
        )
        .route( 
            "/register", 
            post(handler::create_user)
        );
    
    router
}