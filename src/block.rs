use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use ed25519_dalek::Signature;
use crate::wallet::Wallet; // Assuming wallet.rs from Phase 3

// Transaction is now a self-contained, verifiable unit of action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String, // Sender's public key
    pub recipient: String,
    pub amount: u64, // Use integers for currency to avoid float precision issues
    pub nonce: u64, // Sequential number to prevent replay attacks
    pub hash: String,
    pub signature: Signature,
}

impl Transaction {
    // Constructor now requires the signing wallet to ensure authenticity.
    pub fn new(sender_wallet: &Wallet, recipient: String, amount: u64, nonce: u64) -> Self {
        let sender = sender_wallet.public_key_hex();
        
        let mut tx_data_hasher = Sha256::new();
        tx_data_hasher.update(sender.as_bytes());
        tx_data_hasher.update(recipient.as_bytes());
        tx_data_hasher.update(&amount.to_le_bytes());
        tx_data_hasher.update(&nonce.to_le_bytes());
        let hash = format!("{:x}", tx_data_hasher.finalize());

        let signature = sender_wallet.sign(hash.as_bytes());

        Self {
            sender,
            recipient,
            amount,
            nonce,
            hash,
            signature,
        }
    }
}

// Implementations for Hash, PartialEq, Eq to use Transaction in a HashSet
impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
impl Eq for Transaction {}


// Block and BlockHeader remain structurally the same as in Phase 3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub id: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub validator_pubkey: String,
    pub transactions_hash: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub signature: Signature, // The validator's signature of the block header hash
}

impl Block {
    pub fn calculate_header_hash(&self) -> String {
        let header_as_json = serde_json::to_string(&self.header).expect("Failed to serialize block header");
        let mut hasher = Sha256::new();
        hasher.update(header_as_json.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn hash_transactions(transactions: &[Transaction]) -> String {
        let transactions_as_json = serde_json::to_string(transactions).expect("Failed to serialize transactions");
        let mut hasher = Sha256::new();
        hasher.update(transactions_as_json.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
