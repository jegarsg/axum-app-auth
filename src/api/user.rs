use axum::{extract::State, Json};
use crate::service::user_service;
use crate::domain::user::{User, RegisterUser};
use sqlx::PgPool;


pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>
) -> Result<Json<User>, String> {
    match user_service::register_user(&pool, payload).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err),
    }
}