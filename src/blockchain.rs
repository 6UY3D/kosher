use crate::block::{Block, Transaction};
use crate::evm::process_transaction;
use crate::errors::NodeError;
use revm::{
    primitives::{AccountInfo, Bytecode, B160, U256 as RevmU256},
    db::{CacheDB, EmptyDB},
    EVM,
};
use std::collections::{HashMap, HashSet};

// AccountState is now EVM-compatible.
#[derive(Debug, Clone, Default)]
pub struct AccountState {
    pub nonce: u64,
    pub balance: RevmU256,
    pub bytecode: Option<Bytecode>,
    pub storage: HashMap<RevmU256, RevmU256>,
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    validator_set: HashSet<String>,
    // The state is no longer directly managed here, it will be managed via the EVM's database.
}

impl Blockchain {
    // ... new() function ...
    
    /// Validates a block by re-executing its transactions against the state of the previous block.
    pub fn validate_and_add_block(&mut self, block: Block) -> Result<(), NodeError> {
        // --- Setup EVM and DB for validation ---
        let mut cache_db = CacheDB::new(EmptyDB::default());
        // 1. Populate the cache_db with the state from the *end* of the previous block.
        // This is a complex step that requires iterating all previous blocks or using state snapshots.
        // For this example, we assume this is done.

        let mut evm = EVM::new();
        evm.database(cache_db);

        // --- Execute Transactions ---
        for tx in &block.transactions {
            // Set up EVM environment for this block (timestamp, block number, etc.)
            evm.env.block.number = RevmU256::from(block.header.id);
            evm.env.block.timestamp = RevmU256::from(block.header.timestamp);
            
            process_transaction(self, tx, &mut evm)?;
        }
        
        // Block is valid, add it to the chain.
        self.blocks.push(block);
        Ok(())
    }
}    /// Performs comprehensive validation of a block and all its transactions.
    fn is_block_valid(&self, block: &Block) -> Result<(), NodeError> {
        let previous_block = self.blocks.last().ok_or_else(|| NodeError::Blockchain("Genesis block not found".into()))?;

        // --- Block Header Validation ---
        if block.header.id != previous_block.header.id + 1 {
            return Err(NodeError::Blockchain("Invalid block ID".into()));
        }
        if block.header.previous_hash != previous_block.calculate_header_hash() {
            return Err(NodeError::Blockchain("Previous hash mismatch".into()));
        }

        // --- PoA Validator Validation ---
        if !self.validator_set.contains(&block.header.validator_pubkey) {
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
