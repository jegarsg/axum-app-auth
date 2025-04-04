use sqlx::{PgPool, types::Uuid as SqlxUuid};
use crate::{
    domain::user::{User, RegisterUser},
    error::AppError,
};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand::rngs::OsRng;
use tracing::info;

pub async fn create_user(
    pool: &PgPool,
    new_user: &RegisterUser,
) -> Result<User, AppError> {
    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(new_user.password.as_bytes(), &salt)
        .map_err(|e| {
            AppError::ValidationError(format!("Password hashing failed: {}", e))
        })?
        .to_string();

    // Insert user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (
            user_id, email, phone, user_name, full_name,
            password, is_active, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
        RETURNING *
        "#
    )
    .bind(SqlxUuid::new_v4())
    .bind(&new_user.email)
    .bind(&new_user.phone)
    .bind(&new_user.user_name)
    .bind(&new_user.full_name)
    .bind(hashed_password)
    .bind(true) // is_active
    .fetch_one(pool)
    .await
    .map_err(|e| {
        info!("Database error: {}", e);
        AppError::DatabaseError(anyhow::anyhow!(e))
    })?;

    Ok(user)
}