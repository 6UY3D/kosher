use crate::block::{Block, Transaction};
use crate::errors::NodeError;
use crate::wallet::Wallet;
use std::collections::{HashMap, HashSet};
use ed25519_dalek::Signature;
use chrono::Utc;
use revm::primitives::{U256 as RevmU256, Bytecode};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct AccountState {
    pub nonce: u64,
    pub balance: RevmU256,
    pub bytecode: Option<Bytecode>,
    pub storage: HashMap<RevmU256, RevmU256>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub validator_set: HashSet<String>,
    pub state: HashMap<String, AccountState>,
}

impl Blockchain {
    pub fn new(validators: HashSet<String>) -> Self {
        let genesis_block = Block {
            header: crate::block::BlockHeader {
                id: 0,
                timestamp: Utc::now().timestamp(),
                previous_hash: "0".repeat(64),
                validator_pubkey: "system".to_string(),
                transactions_hash: "0".repeat(64),
            },
            transactions: vec![],
            signature: Signature::from_bytes(&[0; 64]).unwrap(),
        };

        Self {
            blocks: vec![genesis_block],
            validator_set: validators,
            state: HashMap::new(),
        }
    }

    /// The core block validation logic. This function is now more focused.
    /// It validates a block and, if valid, returns the new state. The caller is responsible for committing it.
    pub fn validate_and_add_block(&mut self, block: Block) -> Result<(), NodeError> {
        // The validation logic using EVM would be here, as in the previous phase.
        // For brevity, we are showing the simplified validation logic.
        self.is_block_valid(&block)?;

        // Update state based on transactions
        for tx in &block.transactions {
            // This part would be replaced by the EVM state transition logic
            // For example:
            // let mut evm_db = self.build_evm_db();
            // process_transaction(&mut evm, tx);
            // self.state = evm_db.into_state();
        }

        self.blocks.push(block);
        Ok(())
    }
    
    fn is_block_valid(&self, block: &Block) -> Result<(), NodeError> {
        let previous_block = self.blocks.last().ok_or_else(|| NodeError::Blockchain("Genesis block not found".into()))?;

        if block.header.id != previous_block.header.id + 1 {
            return Err(NodeError::Blockchain("Invalid block ID".into()));
        }
        if block.header.previous_hash != previous_block.calculate_header_hash() {
            return Err(NodeError::Blockchain("Previous hash mismatch".into()));
        }
        if !self.validator_set.contains(&block.header.validator_pubkey) {
            return Err(NodeError::Blockchain("Validator not in the approved set".into()));
        }
        
        // ... further validation ...
        
        Ok(())
    }
}        if !self.validator_set.contains(&block.header.validator_pubkey) {
            return Err(NodeError::Blockchain("Validator not in the approved set".into()));
        }
        let message = block.calculate_header_hash();
        if !Wallet::verify_signature(&block.header.validator_pubkey, message.as_bytes(), &block.signature) {
            return Err(NodeError::Blockchain("Invalid block signature".into()));
        }

        // --- Transaction Validation ---
        for tx in &block.transactions {
            self.is_transaction_valid(tx)?;
        }

        Ok(())
    }

    /// Validates an individual transaction against the current state.
    pub fn is_transaction_valid(&self, tx: &Transaction) -> Result<(), NodeError> {
        if !Wallet::verify_signature(&tx.sender, tx.hash.as_bytes(), &tx.signature) {
            return Err(NodeError::Blockchain(format!("Invalid signature on tx {}", tx.hash)));
        }

        let sender_state = self.state.get(&tx.sender).ok_or_else(|| NodeError::Blockchain(format!("Sender {} not found", tx.sender)))?;

        if sender_state.balance < tx.amount {
            return Err(NodeError::Blockchain(format!("Insufficient funds for sender {}", tx.sender)));
        }

        if sender_state.nonce != tx.nonce {
            return Err(NodeError::Blockchain(format!("Invalid nonce for sender {}. Expected: {}, got: {}", tx.sender, sender_state.nonce, tx.nonce)));
        }

        Ok(())
    }
}        let message = block.calculate_header_hash();
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
