use crate::block::{Block, BlockHeader, Transaction};
use crate::wallet::Wallet;
use chrono::Utc;
use std::collections::HashSet;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    // Use a HashSet for efficient validator lookup
    validator_set: HashSet<String>,
}

impl Blockchain {
    /// Creates a new blockchain, initializing it with a set of approved validators.
    pub fn new(validators: HashSet<String>) -> Self {
        // The genesis block is special and doesn't require a real signature
        let genesis_block = Block {
            header: BlockHeader {
                id: 0,
                timestamp: Utc::now().timestamp(),
                previous_hash: "0".repeat(64),
                validator_pubkey: "system".to_string(),
                transactions_hash: "0".repeat(64),
            },
            transactions: vec![],
            // A dummy signature for the genesis block
            signature: Signature::from_bytes(&[0; 64]).unwrap(),
        };

        Self {
            blocks: vec![genesis_block],
            validator_set: validators,
        }
    }

    /// Attempts to add a block to the chain after rigorous validation.
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        if self.is_block_valid(&block) {
            self.blocks.push(block);
            Ok(())
        } else {
            Err("Block validation failed".to_string())
        }
    }

    /// The core validation logic for the PoA consensus.
    fn is_block_valid(&self, block: &Block) -> bool {
        let previous_block = self.blocks.last().unwrap();

        // 1. Check if the validator is in the approved set
        if !self.validator_set.contains(&block.header.validator_pubkey) {
            println!("Validation Error: Validator not in the approved set.");
            return false;
        }

        // 2. Check if the previous_hash is correct
        if block.header.previous_hash != previous_block.calculate_header_hash() {
            println!("Validation Error: Previous hash does not match.");
            return false;
        }

        // 3. Verify the validator's signature
        let message = block.calculate_header_hash();
        if !Wallet::verify_signature(&block.header.validator_pubkey, message.as_bytes(), &block.signature) {
            println!("Validation Error: Invalid block signature.");
            return false;
        }

        // Add more checks here (e.g., timestamp validity, transaction signatures)

        true
    }

    // --- Persistence functions (save_to_file, load_from_file) remain the same ---
    // Note: You would adapt load_from_file to also take an initial validator set
    // or store the set within the chain.json file itself. For simplicity, we'll
    // re-initialize the validator set on each run in main.rs.
    pub fn save_to_file(&self, file_path: &str) -> Result<(), Error> {
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize blockchain");
        fs::write(file_path, data)
    }

    pub fn load_from_file(file_path: &str, validators: HashSet<String>) -> Self {
        match fs::read_to_string(file_path) {
            Ok(data) => {
                let mut chain: Blockchain = serde_json::from_str(&data).expect("Failed to deserialize");
                chain.validator_set = validators; // Ensure validator set is up-to-date
                chain
            },
            Err(e) if e.kind() == ErrorKind::NotFound => {
                println!("No existing blockchain found. Creating a new one.");
                Self::new(validators)
            },
            Err(e) => panic!("Failed to load blockchain file: {}", e),
        }
    }
}    pub fn load_from_file(file_path: &str) -> Self {
        match fs::read_to_string(file_path) {
            Ok(data) => serde_json::from_str(&data).expect("Failed to deserialize blockchain data"),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                println!("No existing blockchain found. Creating a new one.");
                Self::new()
            },
            Err(e) => panic!("Failed to load blockchain file: {}", e),
        }
    }
}
