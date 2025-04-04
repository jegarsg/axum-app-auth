// src/service/user_service.rs
use crate::{
    domain::user::{User, RegisterUser},  // Explicitly import User
    error::AppError,
    repository::user_repository,
};
use anyhow::Context;
use sqlx::PgPool;

pub async fn register_user(
    pool: &PgPool,
    user_data: RegisterUser,
) -> Result<User, AppError> {
    validate_user_input(&user_data)?;
    
    let user = user_repository::create_user(pool, &user_data)
        .await
        .context("Failed to create user")?;

    Ok(user)
}

fn validate_user_input(user: &RegisterUser) -> Result<(), AppError> {
    if !user.email.contains('@') {
        return Err(AppError::ValidationError("Invalid email format".into()));
    }
    if user.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".into(),
        ));
    }
    Ok(())
}