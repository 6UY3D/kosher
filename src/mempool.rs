use crate::block::Transaction;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Mempool {
    transactions: HashSet<Transaction>,
}

impl Mempool {
    /// Adds a transaction to the mempool.
    /// Returns true if the transaction was new, false otherwise.
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        self.transactions.insert(tx)
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
