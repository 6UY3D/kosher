use crate::block::{Block, BlockHeader, Transaction};
use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use crate::p2p::ChainMessage;
use crate::wallet::Wallet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use chrono::Utc;

const BLOCK_PROPOSAL_INTERVAL_SECONDS: u64 = 15;

/// The validator service is responsible for proposing new blocks.
pub struct ValidatorService {
    wallet: Wallet,
    blockchain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
    p2p_tx: mpsc::Sender<ChainMessage>,
}

impl ValidatorService {
    pub fn new(
        wallet: Wallet,
        blockchain: Arc<Mutex<Blockchain>>,
        mempool: Arc<Mutex<Mempool>>,
        p2p_tx: mpsc::Sender<ChainMessage>,
    ) -> Self {
        Self { wallet, blockchain, mempool, p2p_tx }
    }

    /// Runs the main block proposal loop.
    pub async fn run(&self) {
        println!("[Validator] Starting block proposal service. Interval: {}s", BLOCK_PROPOSAL_INTERVAL_SECONDS);
        let mut interval = tokio::time::interval(Duration::from_secs(BLOCK_PROPOSAL_INTERVAL_SECONDS));

        loop {
            interval.tick().await;
            self.propose_block().await;
        }
    }

    /// The core logic for creating and proposing a single block.
    async fn propose_block(&self) {
        let transactions = {
            let mut mempool = self.mempool.lock().unwrap();
            let txs = mempool.get_transactions(100); // Get up to 100 transactions
            mempool.clear(&txs);
            txs
        };

        if transactions.is_empty() {
            println!("[Validator] No transactions in mempool. Skipping block proposal.");
            return;
        }

        println!("[Validator] Proposing new block with {} transactions...", transactions.len());

        let new_block = {
            let chain = self.blockchain.lock().unwrap();
            let previous_block = chain.blocks.last().unwrap();

            let mut block = Block {
                header: BlockHeader {
                    id: previous_block.header.id + 1,
                    timestamp: Utc::now().timestamp(),
                    previous_hash: previous_block.calculate_header_hash(),
                    validator_pubkey: self.wallet.public_key_hex(),
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
