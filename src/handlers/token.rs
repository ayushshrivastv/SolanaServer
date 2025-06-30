use axum::{response::Json, body::Bytes};
use solana_sdk::instruction::Instruction;
use spl_token::instruction::{initialize_mint, mint_to};
use base64::Engine;
use crate::models::requests::{CreateTokenRequest, MintTokenRequest};
use crate::models::responses::{ApiResponse, InstructionResponse, AccountMeta, TokenCreateResponse};
use crate::utils::validation::{validate_pubkey, validate_decimals, validate_amount};

pub async fn create_token(
    body: Bytes,
) -> Json<ApiResponse<TokenCreateResponse>> {
    // Parse JSON manually
    let request: CreateTokenRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => return Json(ApiResponse::error("Missing required fields".to_string())),
    };

    // Validate inputs
    let mint_authority = match validate_pubkey(&request.payer) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    let mint = match validate_pubkey(&request.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    if let Err(e) = validate_decimals(request.decimals) {
        return Json(ApiResponse::error(e));
    }

    // Create initialize mint instruction
    let instruction = match initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority), // freeze authority (using same as mint authority)
        request.decimals,
    ) {
        Ok(instr) => instr,
        Err(e) => return Json(ApiResponse::error(format!("Failed to create instruction: {}", e))),
    };

    let response = token_create_instruction_to_response(instruction);
    Json(ApiResponse::success(response))
}

pub async fn mint_token(
    body: Bytes,
) -> Json<ApiResponse<InstructionResponse>> {
    // Parse JSON manually
    let request: MintTokenRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => return Json(ApiResponse::error("Missing required fields".to_string())),
    };

    // Validate inputs
    let mint = match validate_pubkey(&request.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    let destination = match validate_pubkey(&request.destination) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    let authority = match validate_pubkey(&request.authority) {
        Ok(pubkey) => pubkey,
        Err(e) => return Json(ApiResponse::error(e)),
    };

    if let Err(e) = validate_amount(request.amount) {
        return Json(ApiResponse::error(e));
    }

    // Create mint to instruction
    let instruction = match mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[&authority], // signers
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

fn token_create_instruction_to_response(instruction: Instruction) -> TokenCreateResponse {
    // For token create, return the first account (mint account) as a single object
    let mint_account = instruction.accounts.first().map(|account| AccountMeta {
        pubkey: account.pubkey.to_string(),
        is_signer: account.is_signer,
        is_writable: account.is_writable,
    }).unwrap_or_else(|| AccountMeta {
        pubkey: "11111111111111111111111111111111".to_string(),
        is_signer: false,
        is_writable: true,
    });

    TokenCreateResponse {
        program_id: instruction.program_id.to_string(),
        accounts: mint_account,
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&instruction.data),
    }
} 
