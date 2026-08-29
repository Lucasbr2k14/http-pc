use std::sync::Arc;
use axum::Router;

use crate::AppState;
use crate::fronend;


pub fn router( state: Arc<AppState> ) -> Router {
    let router = Router::new()
        .merge(fronend::routes::routes())
        .with_state(state);
    
    router
}