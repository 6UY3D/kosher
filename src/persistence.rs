use crate::blockchain::{Blockchain, AccountState};
use crate::errors::NodeError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const STATE_FILE: &str = "blockchain_state.json";

/// The data structure that is saved to disk, containing all necessary state.
#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentState {
    blocks: Vec<crate::block::Block>,
    state: HashMap<String, AccountState>,
}

/// Saves the current state of the blockchain to a file.
pub fn save_state(chain: &Blockchain) -> Result<(), NodeError> {
    println!("[Persistence] Saving blockchain state to disk...");
    let state_to_save = PersistentState {
        blocks: chain.blocks.clone(),
        state: chain.state.clone(),
    };

    let data = serde_json::to_string_pretty(&state_to_save)?;
    fs::write(STATE_FILE, data)?;
    println!("[Persistence] State saved successfully. Total blocks: {}", chain.blocks.len());
    Ok(())
}

/// Loads the blockchain state from a file, or creates a new one if it doesn't exist.
pub fn load_or_initialize_state(validators: HashSet<String>) -> Result<Blockchain, NodeError> {
    let path = Path::new(STATE_FILE);
    if path.exists() {
        println!("[Persistence] Found existing state file. Loading from disk...");
        let data = fs::read_to_string(path)?;
        let loaded_state: PersistentState = serde_json::from_str(&data)?;
        
        let mut chain = Blockchain::new(validators);
        chain.blocks = loaded_state.blocks;
        chain.state = loaded_state.state;
        
        println!("[Persistence] State loaded successfully. Current block height: {}", chain.blocks.len() - 1);
        Ok(chain)
    } else {
        println!("[Persistence] No state file found. Initializing a new blockchain.");
        Ok(Blockchain::new(validators))
    }
}
