pub mod config;
pub mod error;
pub mod handlers;
pub mod models;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/pastes", post(handlers::create_paste))
        .route("/pastes/:id", get(handlers::get_paste))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}
