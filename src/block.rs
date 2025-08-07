use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use ed25519_dalek::Signature;
use crate::wallet::Wallet;
use ethers_core::types::{Address, U256};

// An enum to represent the two main types of actions on the chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionAction {
    // A simple transfer of the native token
    Transfer {
        recipient: Address,
        amount: U256,
    },
    // A call to a smart contract (or contract creation if `to` is None)
    Call {
        to: Option<Address>,
        data: Vec<u8>,
        value: U256,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: Address,
    pub action: TransactionAction,
    pub nonce: u64,
    pub hash: String,
    pub signature: Signature,
}

// ... Implementations for Hash, PartialEq, Eq ...

impl Block {
    // ... calculate_header_hash and hash_transactions remain the same ...
}

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
    pub signature: Signature,
}
