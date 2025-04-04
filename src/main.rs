use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use crate::config::database::establish_connection;
use crate::router::create_router;
use crate::service::user_service::*;


mod config;
mod domain;
mod repository;
mod service;
mod api;
mod router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = establish_connection().await;

    let app = create_router(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
