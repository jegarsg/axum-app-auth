use axum::{Router};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper_util::server::Server;
use hyper_util::service::service_fn;
use hyper::body::Body;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod config;
mod domain;
mod repository;
mod service;
mod api;
mod router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = config::database::establish_connection().await;
    let app = router::create_router(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}