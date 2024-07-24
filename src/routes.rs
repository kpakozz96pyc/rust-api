use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

use crate::handlers::kills_handlers::{create_kill, get_kill, list_kills};
use crate::AppState;

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/v1/kills_handlers", posts_routes(state.clone()))
        .fallback(handler_404)
}

async fn root() -> &'static str {
    "Server is running!"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

fn posts_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_kill))
        .route("/", get(list_kills))
        .route("/:id", get(get_kill))
        .with_state(state)
}
