use crate::{
    app_state::AppState,
    handler::{
        handle_create_circle, handle_fetch_circle, handle_get_version, handle_update_circle,
    },
};

use axum::{
    routing::{get, post, put},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .without_v07_checks()
        .route("/version", get(handle_get_version))
        .route("/circle", get(handle_fetch_circle))
        .route("/circle/:id", get(handle_fetch_circle))
        .route("/circle", post(handle_create_circle))
        .route("/circle/:id", put(handle_update_circle))
}
