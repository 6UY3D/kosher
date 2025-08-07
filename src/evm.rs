use crate::blockchain::{Blockchain, AccountState};
use crate::block::Transaction;
use crate::errors::NodeError;
use revm::{
    primitives::{AccountInfo, Bytecode, B160, U256 as RevmU256, TransactTo},
    db::{CacheDB, EmptyDB},
    EVM,
};
use ethers_core::types::{Address, U256};

/// Processes a transaction using the EVM.
/// This function will modify the state in the provided `CacheDB`.
pub fn process_transaction(
    blockchain: &Blockchain,
    transaction: &Transaction,
    evm: &mut EVM<CacheDB<EmptyDB>>,
) -> Result<(), NodeError> {
    
    evm.tx_env.caller = transaction.sender.into();
    evm.tx_env.nonce = Some(transaction.nonce);

    match &transaction.action {
        crate::block::TransactionAction::Transfer { recipient, amount } => {
            evm.tx_env.transact_to = TransactTo::Call((*recipient).into());
            evm.tx_env.value = RevmU256::from_limbs(amount.0);
            evm.tx_env.data = bytes::Bytes::new();
        }
        crate::block::TransactionAction::Call { to, data, value } => {
            evm.tx_env.transact_to = match to {
                Some(addr) => TransactTo::Call((*addr).into()),
                None => TransactTo::Create,
            };
            evm.tx_env.data = data.clone().into();
            evm.tx_env.value = RevmU256::from_limbs(value.0);
        }
    }
    
    // Execute the transaction.
    let result = evm.transact_commit();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(NodeError::Blockchain(format!("EVM transaction failed: {:?}", e))),
    }
}
