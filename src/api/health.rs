use axum::{Json, response::IntoResponse};
use serde_json::json;

/// Health check handler
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Service is running"
    }))
}
