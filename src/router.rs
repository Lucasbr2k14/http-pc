use std::sync::Arc;
use axum::Router;

use crate::AppState;
use crate::fronend;
use crate::user::user_routes;

pub fn router( state: Arc<AppState> ) -> Router {
    let router = Router::new()
        .merge(fronend::routes::routes())
        .merge(user_routes::router_web())
        .with_state(state);
    
    router
}