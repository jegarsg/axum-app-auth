use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::api::user::register;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/user/register", post(register))
        .with_state(pool)
}
