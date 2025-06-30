use axum::{response::Json, http::StatusCode};
use solana_sdk::signer::{keypair::Keypair, Signer};
use crate::models::responses::{ApiResponse, KeypairResponse};

pub async fn generate_keypair() -> Result<Json<ApiResponse<KeypairResponse>>, StatusCode> {
    let keypair = Keypair::new();
    
    let response = KeypairResponse {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    };
    
    Ok(Json(ApiResponse::success(response)))
} 
