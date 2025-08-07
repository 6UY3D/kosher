use crate::config::ApiConfig;
use crate::mempool::{Mempool, MempoolError};
use crate::block::Transaction;
use crate::p2p::ChainMessage;

use axum::{
    routing::post, http::StatusCode, Json, Router, extract::State,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tower_governor::{
    governor::{Governor, GovernorConfigBuilder},
};

#[derive(Clone)]
pub struct AppState {
    pub mempool: Arc<Mutex<Mempool>>,
    pub p2p_tx: mpsc::Sender<ChainMessage>,
}

pub async fn run_api(config: ApiConfig, state: AppState) {
    let governor_config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        .route("/transaction", post(handle_transaction))
        .with_state(state)
        .layer(tower::ServiceBuilder::new().layer(Governor::new(&governor_config)));

    let addr: SocketAddr = config.listen_address.parse().expect("Invalid API listen address");
    println!("[API] Server with rate limiting listening on {}", addr);
    
    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await {
        eprintln!("[API] Server error: {}", e);
    }
}

async fn handle_transaction(
    State(state): State<AppState>,
    Json(tx): Json<Transaction>,
) -> (StatusCode, String) {
    let mut mempool = state.mempool.lock().unwrap();
    
    match mempool.add_transaction(tx.clone()) {
        Ok(_) => {
            println!("[API] Accepted new transaction: {}", tx.hash);
            if state.p2p_tx.try_send(ChainMessage::Transaction(tx)).is_err() {
                eprintln!("[API] Warning: P2P channel is full. Transaction not gossiped immediately.");
            }
            (StatusCode::OK, "Transaction accepted".to_string())
        }
        Err(MempoolError::PoolFull) => {
            (StatusCode::SERVICE_UNAVAILABLE, "Mempool is full".to_string())
        }
        Err(MempoolError::AlreadyExists) => {
            (StatusCode::BAD_REQUEST, "Transaction already in mempool".to_string())
        }
    }
}
