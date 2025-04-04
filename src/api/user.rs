use axum::{
    extract::State,
    response::IntoResponse, // Import IntoResponse
    Json,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    domain::user::RegisterUser,
    error::AppError,
    service::user_service,
};

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = user_service::register_user(&pool, payload).await?;

    Ok(Json(json!({
        "success": true,
        "data": user
    })))
}
