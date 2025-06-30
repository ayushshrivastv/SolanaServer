use axum::{
    routing::{post, get},
    Router,
    response::Json,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use std::env;

mod handlers;
mod models;
mod utils;

use handlers::{keypair, message, token, transfer};
use models::responses::ApiResponse;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        // Keypair operations
        .route("/keypair", get(keypair::generate_keypair).post(keypair::generate_keypair))
        
        // Message operations
        .route("/message/sign", post(message::sign_message))
        .route("/message/verify", post(message::verify_message))
        
        // Token operations
        .route("/token/create", post(token::create_token))
        .route("/token/mint", post(token::mint_token))
        
        // Transfer operations
        .route("/send/sol", post(transfer::send_sol))
        .route("/send/token", post(transfer::send_token))
        
        // Health check endpoint
        .route("/", get(health_check))
        .route("/health", get(health_check))
        
        // Fallback for 404s - must be last
        .fallback(handle_404)
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        );

    // Get port from environment variable or default to 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT value");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    println!("🚀 Solana HTTP Server running on http://0.0.0.0:{}", port);
    
    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Solana HTTP Server is running!".to_string()))
}

async fn handle_404() -> Json<ApiResponse<String>> {
    Json(ApiResponse::error("Endpoint not found".to_string()))
}
