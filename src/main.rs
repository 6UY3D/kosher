mod block;
mod blockchain;

use block::Transaction;
use blockchain::Blockchain;

const CHAIN_FILE: &str = "chain.json";

fn main() {
    // Load the blockchain from disk, or create it if it doesn't exist.
    let mut kosher_chain = Blockchain::load_from_file(CHAIN_FILE);

    println!("--- Kosher Chain: Phase 2 ---");
    println!("Blockchain loaded successfully. Current block count: {}", kosher_chain.blocks.len());

    // Let's create a new transaction and add a new block.
    // In a real system, the validator would be a cryptographic public key.
    let new_transactions = vec![
        Transaction {
            sender: "rabbi_A".to_string(),
            recipient: "rabbi_B".to_string(),
            amount: 50.0,
        },
        Transaction {
            sender: "community_fund".to_string(),
            recipient: "charity_xyz".to_string(),
            amount: 180.0,
        },
    ];

    println!("\nAdding a new block with {} transactions...", new_transactions.len());
    kosher_chain.add_block(new_transactions, "validator_node_1".to_string());
    
    println!("New block added successfully!");
    println!("Current block count: {}", kosher_chain.blocks.len());
    
    // Print the latest block
    if let Some(latest_block) = kosher_chain.blocks.last() {
        println!("\nLatest Block Details:");
        println!("{:#?}", latest_block);
        println!("Latest Block Hash: {}", latest_block.calculate_hash());
    }

    // Save the updated blockchain state to disk.
    println!("\nSaving blockchain state to '{}'...", CHAIN_FILE);
    if let Err(e) = kosher_chain.save_to_file(CHAIN_FILE) {
        eprintln!("Error saving blockchain: {}", e);
    } else {
        println!("Save successful.");
    }
}
