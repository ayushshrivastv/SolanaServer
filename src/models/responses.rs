use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolTransferResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccountMeta {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenTransferResponse {
    pub program_id: String,
    pub accounts: Vec<TokenAccountMeta>,
    pub instruction_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenCreateResponse {
    pub program_id: String,
    pub accounts: AccountMeta, // Single account object as per spec
    pub instruction_data: String,
} 
