use std::collections::{HashSet, HashMap};
use std::fs;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::signal;

mod block;
mod blockchain;
mod wallet;
mod p2p;
mod mempool;
mod xrpl_witness;
mod errors;
mod config;

use config::Config;
use errors::NodeError;

#[tokio::main]
async fn main() -> Result<(), NodeError> {
    println!("--- Kosher Chain Node Starting ---");

    // --- 1. Load Configuration ---
    let config = Config::load("config.toml")?;
    println!("Configuration loaded successfully.");

    // --- 2. Initialize State from Config ---
    let validators_content = fs::read_to_string(&config.chain.validators_file)?;
    let validators_data: HashMap<String, Vec<String>> = serde_json::from_str(&validators_content)?;
    let validator_set: HashSet<String> = validators_data.get("validators").unwrap().iter().cloned().collect();

    let blockchain = Arc::new(Mutex::new(blockchain::Blockchain::new(validator_set)));
    let mempool = Arc::new(Mutex::new(mempool::Mempool::new()));
    // PeerManager and other P2P components would be initialized here...

    // --- 3. Communication Channels ---
    let (p2p_tx, _p2p_rx) = mpsc::channel(100);

    // --- 4. Spawning Concurrent Tasks ---

    // TODO: Create the API task
    // let api_state = api::AppState { mempool: mempool.clone(), p2p_tx: p2p_tx.clone() };
    // let api_task = tokio::spawn(api::run_api(config.api, api_state));
    // println!("API service task spawned.");

    // TODO: Create the P2P network task
    // let p2p_task = tokio::spawn(p2p::run_p2p_network(...));
    // println!("P2P service task spawned.");

    // Create the XRPL Witness task
    let witness_task = tokio::spawn(xrpl_witness::run_xrpl_witness(config.witness));
    println!("XRPL Witness service task spawned.");
    
    // --- 5. Graceful Shutdown ---
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("\nCtrl-C received. Shutting down node gracefully...");
            // Abort running tasks
            // api_task.abort();
            // p2p_task.abort();
            witness_task.abort();
            println!("All services stopped.");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
    
    Ok(())
}                            // e.g., penalize for duplicate or invalid transactions.
                            println!("[Gossip] Received transaction {} from peer {}", tx.hash, source_peer);
                            // mempool.lock().unwrap().add_transaction(tx);
                        }
                        Err(_) => {
                            println!("[Gossip] ❌ Received malformed message from {}. Penalizing peer.", source_peer);
                            let should_ban = peer_manager.lock().unwrap().penalize_peer(&source_peer, 25);
                            if should_ban {
                                swarm.ban_peer(source_peer);
                            }
                        }
                    }
                }
                _ => {
                    // Handle other events like Mdns discovery...
                }
            }
        }
    }
}    println!("User state after tx: {:?}", kosher_chain.state.get(&user_wallet.public_key_hex()));


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
