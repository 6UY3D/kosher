// ... imports ...
use crate::block::{Block, Transaction}; // Add Transaction

// The messages we'll send across the network.
#[derive(Debug, Serialize, Deserialize)]
pub enum ChainMessage {
    Block(Block),
    Transaction(Transaction), // New variant for gossiping transactions
}

// Topics for our gossipsub protocol
pub const CHAIN_TOPIC: Topic = Topic::new("kosher-chain-blocks");
pub const TRANSACTION_TOPIC: Topic = Topic::new("kosher-chain-transactions");

// ... rest of the file remains the same ...
