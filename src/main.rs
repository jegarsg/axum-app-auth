use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

mod config;
mod domain;
mod repository;
mod service;
mod api;
mod router;
mod error;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("axum_app=info".parse().unwrap()))
        .init();

    info!("Starting application initialization");
    
    dotenv().ok();
    info!("Environment variables loaded");

    // Directly use the pool since establish_connection returns Pool, not Result
    let pool = config::database::establish_connection().await;
    info!("Database connection established");

    let app = router::create_router(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        error!("Failed to bind to address {}: {}", addr, e);
        std::process::exit(1);
    });

    info!("🚀 Server running at http://{}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
    }
}