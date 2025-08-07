mod block;
mod blockchain;
mod wallet;
mod p2p;
mod mempool; // Add mempool module

use block::Transaction;
use blockchain::Blockchain;
use mempool::Mempool;
use p2p::{ChainBehaviour, ChainBehaviourEvent, ChainMessage, CHAIN_TOPIC, TRANSACTION_TOPIC};

use libp2p::{gossipsub, Swarm};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

// API related imports
use axum::{
    routing::post,
    http::StatusCode,
    Json, Router, extract::State,
};
use std::net::SocketAddr;

// Define a shared state for our API
#[derive(Clone)]
struct AppState {
    mempool: Arc<Mutex<Mempool>>,
    p2p_tx: mpsc::Sender<ChainMessage>, // Channel to send messages to the p2p task
}

#[tokio::main]
async fn main() {
    // --- 1. Setup Shared State ---
    let blockchain = Arc::new(Mutex::new(Blockchain::load_from_file(/*..args..*/)));
    let mempool = Arc::new(Mutex::new(Mempool::default()));
    let (p2p_tx, mut p2p_rx) = mpsc::channel(100);

    // --- 2. Start the API Server ---
    let app_state = AppState {
        mempool: mempool.clone(),
        p2p_tx,
    };
    tokio::spawn(run_api(app_state));

    // --- 3. Setup and Run the P2P Swarm ---
    let mut swarm: Swarm<ChainBehaviour> = /* ... setup code from Phase 4 ... */;
    swarm.behaviour_mut().gossipsub.subscribe(&CHAIN_TOPIC).unwrap();
    swarm.behaviour_mut().gossipsub.subscribe(&TRANSACTION_TOPIC).unwrap();

    // --- 4. Main Event Loop ---
    loop {
        tokio::select! {
            // Handle messages from other tasks (e.g., the API)
            Some(msg) = p2p_rx.recv() => {
                let topic = match msg {
                    ChainMessage::Block(_) => CHAIN_TOPIC.clone(),
                    ChainMessage::Transaction(_) => TRANSACTION_TOPIC.clone(),
                };
                let json_msg = serde_json::to_string(&msg).unwrap();
                swarm.behaviour_mut().gossipsub.publish(topic, json_msg).unwrap();
            }
            // Handle events from the P2P swarm
            event = swarm.select_next_some() => match event {
                // ... other SwarmEvent handlers from Phase 4 ...
                
                // Handle incoming gossip messages
                libp2p::swarm::SwarmEvent::Behaviour(ChainBehaviourEvent::Gossipsub(
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
