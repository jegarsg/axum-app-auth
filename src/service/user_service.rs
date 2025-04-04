use crate::repository::user_repository;
use crate::domain::user::{User, RegisterUser};
use sqlx::PgPool;


pub async fn register_user(pool: &PgPool, new_user: RegisterUser) -> Result<User, String> {
    match user_repository::create_user(pool, &new_user).await {
        Ok(user) => Ok(user),
        Err(err) => {
            eprintln!("Error creating user: {:?}", err);
            Err("Failed to create user".to_string())
        }
    }
}
