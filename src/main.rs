mod block;
mod blockchain;
mod wallet;

use block::{Block, BlockHeader, Transaction};
use blockchain::{Blockchain, AccountState};
use wallet::Wallet;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("--- Kosher Chain: Phase A (Hardened) ---");

    // 1. Setup wallets for a user and a validator
    let user_wallet = Wallet::new();
    let validator_wallet = Wallet::new();
    let recipient_pubkey = Wallet::new().public_key_hex();

    // 2. Setup the Blockchain with the validator and initial user state
    let mut validator_set = HashSet::new();
    validator_set.insert(validator_wallet.public_key_hex());
    
    let mut initial_state = HashMap::new();
    initial_state.insert(
        user_wallet.public_key_hex(),
        AccountState { nonce: 0, balance: 1000 }
    );
    
    // We create a new blockchain manually here for demo purposes.
    let mut kosher_chain = Blockchain {
        blocks: vec![ /* genesis block */ ], // Assume a genesis block exists
        validator_set,
        state: initial_state,
    };

    // 3. User creates a valid transaction (nonce = 0)
    println!("\nUser creating first transaction (nonce 0)...");
    let tx1 = Transaction::new(
        &user_wallet,
        recipient_pubkey.clone(),
        100,
        0 // Correct first nonce
    );
    println!("Transaction Hash: {}", tx1.hash);

    // 4. Validator creates and signs a block with this transaction
    let previous_block = kosher_chain.blocks.last().unwrap();
    let mut block1 = Block {
        header: BlockHeader {
            id: previous_block.header.id + 1,
            timestamp: 0, // Simplified timestamp
            previous_hash: previous_block.calculate_header_hash(),
            validator_pubkey: validator_wallet.public_key_hex(),
            transactions_hash: Block::hash_transactions(&vec![tx1.clone()]),
        },
        transactions: vec![tx1.clone()],
        signature: ed25519_dalek::Signature::from_bytes(&[0; 64]).unwrap(), // Dummy
    };
    let block1_hash = block1.calculate_header_hash();
    block1.signature = validator_wallet.sign(block1_hash.as_bytes());
    
    // 5. Add the valid block to the chain
    println!("\nValidator proposing block with nonce 0 transaction...");
    match kosher_chain.validate_and_add_block(block1) {
        Ok(_) => println!("✅ SUCCESS: Block 1 added."),
        Err(e) => eprintln!("❌ FAILURE: {}", e),
    }
    println!("User state after tx: {:?}", kosher_chain.state.get(&user_wallet.public_key_hex()));


    // 6. **DEMONSTRATE SECURITY**: Attacker tries to replay the same transaction in a new block
    println!("\nAttacker attempting to replay nonce 0 transaction in a new block...");
    let previous_block_2 = kosher_chain.blocks.last().unwrap();
    let mut replay_block = Block {
        header: BlockHeader {
            id: previous_block_2.header.id + 1,
            timestamp: 1,
            previous_hash: previous_block_2.calculate_header_hash(),
            validator_pubkey: validator_wallet.public_key_hex(),
            transactions_hash: Block::hash_transactions(&vec![tx1]), // Re-using tx1
        },
        transactions: vec![tx1],
        signature: ed25519_dalek::Signature::from_bytes(&[0; 64]).unwrap(),
    };
    let replay_block_hash = replay_block.calculate_header_hash();
    replay_block.signature = validator_wallet.sign(replay_block_hash.as_bytes());

    match kosher_chain.validate_and_add_block(replay_block) {
        Ok(_) => eprintln!("❌ FAILURE: Replay attack succeeded! Security flaw!"),
        Err(e) => println!("✅ SUCCESS: Chain correctly rejected the block. Reason: {}", e),
    }
    println!("User state (unchanged): {:?}", kosher_chain.state.get(&user_wallet.public_key_hex()));
}                libp2p::swarm::SwarmEvent::Behaviour(ChainBehaviourEvent::Gossipsub(
                    gossipsub::Event::Message { message, .. }
                )) => {
                    if let Ok(msg) = serde_json::from_slice::<ChainMessage>(&message.data) {
                        match msg {
                            ChainMessage::Block(block) => {
                                // Block handling logic from Phase 4
                            }
                            ChainMessage::Transaction(tx) => {
                                println!("Received new transaction via gossip: {}", tx.hash);
                                mempool.lock().unwrap().add_transaction(tx);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// --- API Server Logic ---
async fn run_api(state: AppState) {
    let app = Router::new()
        .route("/transaction", post(handle_transaction))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API server listening on {}", addr);
    axum::serve(axum::Server::bind(&addr), app).await.unwrap();
}

async fn handle_transaction(
    State(state): State<AppState>,
    Json(tx): Json<Transaction>,
) -> StatusCode {
    let mut mempool = state.mempool.lock().unwrap();
    
    // Add to local mempool
    if mempool.add_transaction(tx.clone()) {
        println!("Accepted new transaction via API: {}", tx.hash);
        // Gossip to network
        state.p2p_tx.send(ChainMessage::Transaction(tx)).await.unwrap();
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST // Transaction already exists
    }
}

/*
NOTE: The `main` function is heavily simplified to show the structure. You would need
to merge this logic with the full code from Phase 4, including P2P setup, user input
for block creation, and the main event loop. The key change is adding the `mempool`
to the shared state and handling the new `ChainMessage::Transaction` variant.

The block creation logic (the "create" command) would now look like this:
1. Lock the mempool.
2. Call `mempool.get_transactions(10)` to get up to 10 pending transactions.
3. Create a new block with these transactions.
4. Gossip the new block.
5. Call `mempool.clear(&transactions_in_block)` to remove them from the mempool.
*/
// ... other module imports
mod xrpl_witness;

// ... other use statements
use xrpl_witness::run_xrpl_witness;

#[tokio::main]
async fn main() {
    // --- 1. Setup Shared State ---
    // ... all setup from Phase 5 ...

    // --- 2. Start the API Server ---
    tokio::spawn(run_api(app_state));

    // --- 3. Start the XRPL Witness Service ---
    tokio::spawn(run_xrpl_witness());

    // --- 4. Setup and Run the P2P Swarm ---
    // ... all P2P setup and main event loop from Phase 5 ...
}
