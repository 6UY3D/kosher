use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};

// A transaction now includes its own hash for uniqueness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub hash: String, // Hash of sender, recipient, and amount
}

impl Transaction {
    // A constructor to ensure the hash is always calculated
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        let mut tx = Self { sender, recipient, amount, hash: String::default() };
        tx.hash = tx.calculate_hash();
        tx
    }
    
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(self.recipient.as_bytes());
        hasher.update(&self.amount.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
}

// Implement Hash and Eq to allow Transaction to be stored in a HashSet
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

// ... Block and BlockHeader structs remain the same ...
