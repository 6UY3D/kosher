use crate::block::{Block, BlockHeader, Transaction};
use chrono::Utc;
use std::fs;
use std::io::{Error, ErrorKind};

// The Blockchain struct holds all the blocks and manages the chain's state.
// We are adding `validators` here in preparation for Phase 3.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    validators: Vec<String>, // List of public keys of approved validators
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block.
    fn new() -> Self {
        let mut validators = Vec::new();
        // For now, we'll add a placeholder "system" validator.
        // This will be replaced with real keys in the next phase.
        validators.push("system".to_string());

        let genesis_block = Block {
            header: BlockHeader {
                id: 0,
                timestamp: Utc::now().timestamp(),
                previous_hash: "0".repeat(64),
                validator: "system".to_string(),
                transactions_hash: "0".repeat(64),
            },
            transactions: vec![],
        };
        
        Self {
            blocks: vec![genesis_block],
            validators,
        }
    }

    /// Adds a new block to the chain.
    pub fn add_block(&mut self, transactions: Vec<Transaction>, validator: String) {
        let previous_block = self.blocks.last().expect("Blockchain should have at least one block");
        let previous_hash = previous_block.calculate_hash();
        
        let new_block = Block {
            header: BlockHeader {
                id: previous_block.header.id + 1,
                timestamp: Utc::now().timestamp(),
                previous_hash,
                validator,
                transactions_hash: Block::hash_transactions(&transactions),
            },
            transactions,
        };

        self.blocks.push(new_block);
    }

    /// Saves the entire blockchain state to a JSON file.
    pub fn save_to_file(&self, file_path: &str) -> Result<(), Error> {
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize blockchain");
        fs::write(file_path, data)
    }

    /// Loads the blockchain from a file, or creates a new one if the file doesn't exist.
    pub fn load_from_file(file_path: &str) -> Self {
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
