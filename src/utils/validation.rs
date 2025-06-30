use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn validate_pubkey(pubkey_str: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(pubkey_str).map_err(|_| "Invalid public key format".to_string())
}

pub fn validate_base58_secret(secret: &str) -> Result<Vec<u8>, String> {
    bs58::decode(secret)
        .into_vec()
        .map_err(|_| "Invalid secret key format".to_string())
}

pub fn validate_amount(amount: u64) -> Result<(), String> {
    // Allow zero amounts as they are valid for various Solana use cases
    Ok(())
}

pub fn validate_decimals(decimals: u8) -> Result<(), String> {
    if decimals > 9 {
        return Err("Decimals must be between 0 and 9".to_string());
    }
    Ok(())
} 
