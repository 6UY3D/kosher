use crate::block::Transaction;
use std::collections::HashSet;

// The maximum number of transactions our mempool will hold.
const MAX_MEMPOOL_SIZE: usize = 5000;

#[derive(Debug)]
pub struct Mempool {
    transactions: HashSet<Transaction>,
    max_size: usize,
}

// Custom error type for adding a transaction to the mempool.
#[derive(Debug, PartialEq)]
pub enum MempoolError {
    PoolFull,
    AlreadyExists,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: HashSet::new(),
            max_size: MAX_MEMPOOL_SIZE,
        }
    }
    
    /// Adds a transaction to the mempool, enforcing size limits.
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), MempoolError> {
        if self.transactions.len() >= self.max_size {
            return Err(MempoolError::PoolFull);
        }
        
        if !self.transactions.insert(tx) {
            return Err(MempoolError::AlreadyExists);
        }
        
        Ok(())
    }

    /// Returns a vector of transactions to be included in a block.
    pub fn get_transactions(&self, count: usize) -> Vec<Transaction> {
        self.transactions.iter().take(count).cloned().collect()
    }
    
    /// Removes transactions that have been included in a block.
    pub fn clear(&mut self, transactions_to_remove: &[Transaction]) {
        for tx in transactions_to_remove {
            self.transactions.remove(tx);
        }
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}
