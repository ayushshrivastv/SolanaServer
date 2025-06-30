use axum::{response::Json, http::StatusCode, extract::rejection::JsonRejection, body::Bytes};
use solana_sdk::{instruction::Instruction, system_instruction};
use spl_token::instruction::transfer;
use base64::Engine;
use crate::models::requests::{SendSolRequest, SendTokenRequest};
use crate::models::responses::{ApiResponse, InstructionResponse, AccountMeta};
use crate::utils::validation::{validate_pubkey, validate_amount};

pub async fn send_sol(
    body: Bytes,
) -> Json<ApiResponse<InstructionResponse>> {
    // Parse JSON manually
    let request: SendSolRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => return Json(ApiResponse::error("Invalid JSON or missing required fields".to_string())),
    };

    // Validate inputs
    let from = match validate_pubkey(&request.from) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    let to = match validate_pubkey(&request.to) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    if let Err(e) = validate_amount(request.lamports) {
        return Json(ApiResponse::error(e));
    }

    // Create SOL transfer instruction
    let instruction = system_instruction::transfer(&from, &to, request.lamports);

    let response = instruction_to_response(instruction);
    Json(ApiResponse::success(response))
}

pub async fn send_token(
    body: Bytes,
) -> Json<ApiResponse<InstructionResponse>> {
    // Parse JSON manually
    let request: SendTokenRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => return Json(ApiResponse::error("Invalid JSON or missing required fields".to_string())),
    };

    // Validate inputs
    let destination = match validate_pubkey(&request.destination) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    let _mint = match validate_pubkey(&request.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    let owner = match validate_pubkey(&request.owner) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    if let Err(e) = validate_amount(request.amount) {
        return Json(ApiResponse::error(e));
    }

    // For SPL token transfer, we need to derive the source token account
    // In a real scenario, you would need to find or derive the actual token accounts
    // For this assignment, we'll assume the owner is the token account for simplicity
    let source = owner; // This is simplified - normally you'd derive the associated token account

    // Create SPL token transfer instruction
    let instruction = match transfer(
        &spl_token::id(),
        &source,
        &destination,
        &owner,
        &[&owner], // signers
        request.amount,
    ) {
        Ok(instr) => instr,
        Err(e) => return Json(ApiResponse::error(format!("Failed to create instruction: {}", e))),
    };

    let response = instruction_to_response(instruction);
    Json(ApiResponse::success(response))
}

fn instruction_to_response(instruction: Instruction) -> InstructionResponse {
    let accounts = instruction
        .accounts
        .iter()
        .map(|account| AccountMeta {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&instruction.data),
    }
} 
