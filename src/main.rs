use axum::{
    routing::{post, get, put, patch, delete, head, options},
    Router,
    response::Json,
    http::StatusCode,
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
        .route("/keypair", 
            post(keypair::generate_keypair)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        
        // Message operations
        .route("/message/sign", 
            post(message::sign_message)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        .route("/message/verify", 
            post(message::verify_message)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        
        // Token operations
        .route("/token/create", 
            post(token::create_token)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        .route("/token/mint", 
            post(token::mint_token)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        
        // Transfer operations
        .route("/send/sol", 
            post(transfer::send_sol)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        .route("/send/token", 
            post(transfer::send_token)
            .get(handle_method_error)
            .put(handle_method_error)
            .patch(handle_method_error)
            .delete(handle_method_error)
            .head(handle_method_error)
            .options(handle_method_error)
        )
        
        // Fallback for everything else
        .fallback(handle_fallback)
        
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

// Handle both 404s and method not allowed with same response format
async fn handle_fallback() -> (StatusCode, Json<ApiResponse<String>>) {
    (StatusCode::OK, Json(ApiResponse::error("Endpoint not found".to_string())))
}

// Handle wrong methods on valid endpoints - return HTTP 200 with JSON error
async fn handle_method_error() -> (StatusCode, Json<ApiResponse<String>>) {
    (StatusCode::OK, Json(ApiResponse::error("Method not allowed".to_string())))
}
