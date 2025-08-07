use crate::block::{Block, BlockHeader, Transaction};
use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::p2p::ChainMessage;
use crate::wallet::Wallet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use tracing::{info, warn, error, debug};

const BLOCK_PROPOSAL_INTERVAL_SECONDS: u64 = 15;

pub struct ValidatorService {
    wallet: Wallet,
    blockchain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
    p2p_tx: mpsc::Sender<ChainMessage>,
}

impl ValidatorService {
    // ... new() function ...

    pub async fn run(&self) {
        info!(interval = %BLOCK_PROPOSAL_INTERVAL_SECONDS, "Starting block proposal service.");
        let mut interval = tokio::time::interval(Duration::from_secs(BLOCK_PROPOSAL_INTERVAL_SECONDS));

        loop {
            interval.tick().await;
            self.propose_block().await;
        }
    }

    async fn propose_block(&self) {
        let transactions = {
            let mut mempool = self.mempool.lock().unwrap();
            let txs = mempool.get_transactions(100);
            if !txs.is_empty() {
                mempool.clear(&txs);
            }
            txs
        };

        if transactions.is_empty() {
            debug!("No transactions in mempool. Skipping block proposal.");
            return;
        }

        info!(num_txs = %transactions.len(), "Proposing new block...");

        let new_block = {
            // ... block creation logic ...
        };

        if let Err(e) = self.p2p_tx.send(ChainMessage::Block(new_block)).await {
            error!("Failed to send block to P2P service: {}", e);
        } else {
            info!("New block sent to P2P network for broadcast.");
        }
    }
}                    validator_pubkey: self.wallet.public_key_hex(),
                    transactions_hash: Block::hash_transactions(&transactions),
                },
                transactions,
                signature: ed25519_dalek::Signature::from_bytes(&[0; 64]).unwrap(),
            };
            
            let block_hash = block.calculate_header_hash();
            block.signature = self.wallet.sign(block_hash.as_bytes());
            block
        };

        // Send the new block to the P2P task to be gossiped to the network.
        if let Err(e) = self.p2p_tx.send(ChainMessage::Block(new_block)).await {
            eprintln!("[Validator] Failed to send block to P2P service: {}", e);
        } else {
            println!("[Validator] New block sent to P2P network for broadcast.");
        }
    }
}
