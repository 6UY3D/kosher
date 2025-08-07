// Add `Signature` from the ed25519_dalek crate
use ed25519_dalek::Signature;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use sha2::{Sha256, Digest};

// Transaction struct remains the same for now
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

// BlockHeader now contains the validator's signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub id: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub validator_pubkey: String, // Public key of the validator
    pub transactions_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub signature: Signature, // The validator's signature of the block header hash
}

impl Block {
    /// Calculates the SHA-256 hash of the block's header.
    /// This is the data that the validator will sign.
    pub fn calculate_header_hash(&self) -> String {
        let header_as_json = serde_json::to_string(&self.header).expect("Failed to serialize block header");
        let mut hasher = Sha256::new();
        hasher.update(header_as_json.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // hash_transactions remains the same
    pub fn hash_transactions(transactions: &[Transaction]) -> String {
        let transactions_as_json = serde_json::to_string(transactions).expect("Failed to serialize transactions");
        let mut hasher = Sha256::new();
        hasher.update(transactions_as_json.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

    /// Creates a simple hash of all transactions to include in the header.
    /// This ensures that the transaction data cannot be altered without invalidating the block hash.
    pub fn hash_transactions(transactions: &[Transaction]) -> String {
        let transactions_as_json = serde_json::to_string(transactions).expect("Failed to serialize transactions");
        let mut hasher = Sha256::new();
        hasher.update(transactions_as_json.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
