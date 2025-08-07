use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use sha2::{Sha256, Digest};

// A simplified transaction structure.
// In a real system, this would be much more complex, including signatures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

// The BlockHeader contains the metadata for the block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub id: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub validator: String, // Public key of the validator who created the block
    pub transactions_hash: String, // A hash of all transactions in the block
}

// The Block combines the header with the actual transaction data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Calculates the SHA-256 hash of the block's header.
    /// This hash serves as the unique identifier for the block.
    pub fn calculate_hash(&self) -> String {
        let header_as_json = serde_json::to_string(&self.header).expect("Failed to serialize block header");
        let mut hasher = Sha256::new();
        hasher.update(header_as_json.as_bytes());
        format!("{:x}", hasher.finalize())
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
