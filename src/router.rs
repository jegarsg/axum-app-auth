use axum::{Router, routing::{get, post}};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(crate::api::health::health_check))
        .route("/api/user/register", post(crate::api::user::register))
        .with_state(pool)
}