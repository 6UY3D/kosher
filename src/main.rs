// Declare the block module
mod block;

use block::{Block, BlockHeader, Transaction};
use chrono::Utc;

fn main() {
    // Create some dummy transactions for the genesis block
    let transactions = vec![
        Transaction {
            sender: "system".to_string(),
            recipient: "rabbi_A".to_string(),
            amount: 1000.0, // Initial allocation
        },
    ];

    // The genesis block has an ID of 0 and no previous hash.
    let genesis_block = Block {
        header: BlockHeader {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: "0".repeat(64), // A string of 64 zeros
            validator: "system".to_string(),
            transactions_hash: Block::hash_transactions(&transactions),
        },
        transactions,
    };

    let genesis_hash = genesis_block.calculate_hash();

    println!("--- Kosher Chain: Phase 1 ---");
    println!("Genesis Block created successfully!");
    println!("{:#?}", genesis_block);
    println!("Genesis Block Hash: {}", genesis_hash);
}
