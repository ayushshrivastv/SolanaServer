use axum::{response::Json, http::StatusCode, extract::rejection::JsonRejection, body::Bytes};
use solana_sdk::signer::{keypair::Keypair, Signer};
use solana_sdk::signature::Signature;
use solana_sdk::pubkey::Pubkey;
use base64::Engine;
use crate::models::requests::{SignMessageRequest, VerifyMessageRequest};
use crate::models::responses::{ApiResponse, SignatureResponse, VerificationResponse};

pub async fn sign_message(
    body: Bytes,
) -> Json<ApiResponse<SignatureResponse>> {
    // Parse JSON manually
    let request: SignMessageRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => return Json(ApiResponse::error("Invalid JSON or missing required fields".to_string())),
    };

    // Validate input
    if request.message.is_empty() || request.secret.is_empty() {
        return Json(ApiResponse::error("Missing required fields".to_string()));
    }

    // Decode the secret key (full keypair bytes)
    let keypair_bytes = match bs58::decode(&request.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid secret key format".to_string())),
    };

    // Create keypair from bytes
    let keypair = match Keypair::from_bytes(&keypair_bytes) {
        Ok(kp) => kp,
        Err(_) => return Json(ApiResponse::error("Invalid secret key".to_string())),
    };

    // Sign the message
    let message_bytes = request.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response = SignatureResponse {
        signature: base64::engine::general_purpose::STANDARD.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: request.message,
    };

    Json(ApiResponse::success(response))
}

pub async fn verify_message(
    body: Bytes,
) -> Json<ApiResponse<VerificationResponse>> {
    // Parse JSON manually
    let request: VerifyMessageRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => return Json(ApiResponse::error("Invalid JSON or missing required fields".to_string())),
    };

    // Validate input
    if request.message.is_empty() || request.signature.is_empty() || request.pubkey.is_empty() {
        return Json(ApiResponse::error("Missing required fields".to_string()));
    }

    // Parse public key
    let pubkey: Pubkey = match request.pubkey.parse() {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid public key format".to_string())),
    };

    // Decode signature
    let signature_bytes = match base64::engine::general_purpose::STANDARD.decode(&request.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid signature format".to_string())),
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return Json(ApiResponse::error("Invalid signature".to_string())),
    };

    // Verify the signature
    let message_bytes = request.message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    let response = VerificationResponse {
        valid: is_valid,
        message: request.message,
        pubkey: request.pubkey,
    };

    Json(ApiResponse::success(response))
} 
