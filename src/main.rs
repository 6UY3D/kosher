mod block;
mod blockchain;
mod wallet;

use block::{Block, BlockHeader, Transaction};
use blockchain::Blockchain;
use wallet::Wallet;
use std::collections::HashSet;

const CHAIN_FILE: &str = "chain.json";

fn main() {
    println!("--- Kosher Chain: Phase 3 ---");

    // 1. Create wallets for the Rabbinic Council validators
    let rabbi_a_wallet = Wallet::new();
    let rabbi_b_wallet = Wallet::new();
    let council_member_c_wallet = Wallet::new();
    let unauthorized_wallet = Wallet::new(); // An outsider

    // 2. Define the authorized validator set using their public keys
    let mut validator_set = HashSet::new();
    validator_set.insert(rabbi_a_wallet.public_key_hex());
    validator_set.insert(rabbi_b_wallet.public_key_hex());
    validator_set.insert(council_member_c_wallet.public_key_hex());

    println!("Authorized Validators (Public Keys):");
    for v in &validator_set {
        println!("- {}", v);
    }

    // 3. Load or create the blockchain with the validator set
    let mut kosher_chain = Blockchain::load_from_file(CHAIN_FILE, validator_set);
    println!("\nBlockchain loaded. Current block count: {}", kosher_chain.blocks.len());

    // 4. A valid validator (Rabbi A) creates a new block
    println!("\nAttempting to add a block by an authorized validator (Rabbi A)...");
    
    let new_transactions = vec![Transaction { sender: "a".into(), recipient: "b".into(), amount: 10.0 }];
    let previous_block = kosher_chain.blocks.last().unwrap();

    let mut valid_block = Block {
        header: BlockHeader {
            id: previous_block.header.id + 1,
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash: previous_block.calculate_header_hash(),
            validator_pubkey: rabbi_a_wallet.public_key_hex(),
            transactions_hash: Block::hash_transactions(&new_transactions),
        },
        transactions: new_transactions,
        signature: Signature::from_bytes(&[0; 64]).unwrap(), // Dummy signature for now
    };
    let block_hash = valid_block.calculate_header_hash();
    valid_block.signature = rabbi_a_wallet.sign(block_hash.as_bytes());

    match kosher_chain.add_block(valid_block) {
        Ok(_) => println!("✅ SUCCESS: Block added by Rabbi A."),
        Err(e) => eprintln!("❌ FAILURE: {}", e),
    }

    // 5. An unauthorized person attempts to create a block
    println!("\nAttempting to add a block by an unauthorized person...");

    let rogue_transactions = vec![Transaction { sender: "x".into(), recipient: "y".into(), amount: 99.0 }];
    let previous_block = kosher_chain.blocks.last().unwrap();

    let mut rogue_block = Block {
        header: BlockHeader {
            id: previous_block.header.id + 1,
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash: previous_block.calculate_header_hash(),
            validator_pubkey: unauthorized_wallet.public_key_hex(),
            transactions_hash: Block::hash_transactions(&rogue_transactions),
        },
        transactions: rogue_transactions,
        signature: Signature::from_bytes(&[0; 64]).unwrap(),
    };
    let rogue_hash = rogue_block.calculate_header_hash();
    rogue_block.signature = unauthorized_wallet.sign(rogue_hash.as_bytes());

    match kosher_chain.add_block(rogue_block) {
        Ok(_) => println!("✅ SUCCESS: Block added by rogue actor."),
        Err(e) => eprintln!("❌ FAILURE: {}. Block was correctly rejected.", e),
    }

    println!("\nFinal block count: {}", kosher_chain.blocks.len());

    // Save the updated chain
    kosher_chain.save_to_file(CHAIN_FILE).unwrap();
}
