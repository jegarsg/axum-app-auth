use sqlx::PgPool;
use crate::domain::user::{User, RegisterUser};
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;


pub async fn create_user(pool: &PgPool, new_user: &RegisterUser) -> Result<User, sqlx::Error> {
    // Generate salt and hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(new_user.password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();
    
    // Insert user into database
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (user_id, email, phone, user_name, full_name, password, is_active, created_at) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW()) RETURNING user_id, email, phone, user_name, full_name, password, is_active, created_by, created_at, modified_by, modified_at"
    )
    .bind(Uuid::new_v4())
    .bind(&new_user.email)
    .bind(&new_user.phone)
    .bind(&new_user.user_name)
    .bind(&new_user.full_name)
    .bind(&hashed_password)
    .bind(true) // is_active
    .fetch_one(pool)
    .await?;

    Ok(user)
}
