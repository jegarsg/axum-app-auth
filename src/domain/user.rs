// src/domain/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub phone: String,
    pub user_name: String,
    pub full_name: String,
    pub password: String,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub modified_by: Option<Uuid>,
    pub modified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub phone: String,
    pub user_name: String,
    pub full_name: String,
    pub password: String,
}
