use axum::{extract::Json as ExtractJson, response::Json, http::StatusCode};
use solana_sdk::instruction::Instruction;
use spl_token::instruction::{initialize_mint, mint_to};
use base64::Engine;
use crate::models::requests::{CreateTokenRequest, MintTokenRequest};
use crate::models::responses::{ApiResponse, InstructionResponse, AccountMeta};
use crate::utils::validation::{validate_pubkey, validate_decimals, validate_amount};

pub async fn create_token(
    ExtractJson(request): ExtractJson<CreateTokenRequest>,
) -> Result<Json<ApiResponse<InstructionResponse>>, StatusCode> {
    // Validate inputs
    let mint_authority = match validate_pubkey(&request.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(Json(ApiResponse::error(e))),
    };

    let mint = match validate_pubkey(&request.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(Json(ApiResponse::error(e))),
    };

    if let Err(e) = validate_decimals(request.decimals) {
        return Ok(Json(ApiResponse::error(e)));
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
        Err(e) => return Ok(Json(ApiResponse::error(format!("Failed to create instruction: {}", e)))),
    };

    let response = instruction_to_response(instruction);
    Ok(Json(ApiResponse::success(response)))
}

pub async fn mint_token(
    ExtractJson(request): ExtractJson<MintTokenRequest>,
) -> Result<Json<ApiResponse<InstructionResponse>>, StatusCode> {
    // Validate inputs
    let mint = match validate_pubkey(&request.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(Json(ApiResponse::error(e))),
    };

    let destination = match validate_pubkey(&request.destination) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(Json(ApiResponse::error(e))),
    };

    let authority = match validate_pubkey(&request.authority) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(Json(ApiResponse::error(e))),
    };

    if let Err(e) = validate_amount(request.amount) {
        return Ok(Json(ApiResponse::error(e)));
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
        Err(e) => return Ok(Json(ApiResponse::error(format!("Failed to create instruction: {}", e)))),
    };

    let response = instruction_to_response(instruction);
    Ok(Json(ApiResponse::success(response)))
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
