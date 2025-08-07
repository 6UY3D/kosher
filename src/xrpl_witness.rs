use xrpl_rust::client::{Client, JsonRpcClient};
use xrpl_rust::models::requests::Subscribe;
use xrpl_rust::models::streams::StreamMessage;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

// The public address of the multisignature door account on the XRPL.
const DOOR_ACCOUNT: &str = "rDoorAccountAddressOnXrpl..."; // Replace with your actual address

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInfo {
    pub from: String,       // Sender's XRPL address
    pub amount: String,     // Amount of XRP deposited (as a string)
    pub l2_destination: u64,// Destination on Kosher chain, from DestinationTag
}

/// Runs the witness service that listens for deposits on the XRPL.
pub async fn run_xrpl_witness() {
    println!("[XRPL Witness] Starting...");
    let client = JsonRpcClient::new("wss://s1.ripple.com:51234".parse().unwrap())
        .expect("Cannot create client");

    let sub = Subscribe::new()
        .accounts(vec![DOOR_ACCOUNT.to_string()])
        .build()
        .unwrap();
    
    let mut stream = client.subscribe(&sub).await.expect("Failed to subscribe");
    println!("[XRPL Witness] Subscribed to door account: {}", DOOR_ACCOUNT);

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(StreamMessage::Transaction(tx)) => {
                println!("[XRPL Witness] Received a transaction...");
                if let Some(deposit) = parse_deposit_transaction(&tx) {
                    println!("[XRPL Witness] ✅ Valid deposit detected: {:?}", deposit);
                    // In a real system, this would trigger a consensus vote
                    // among validators to mint the corresponding kXRP on the L2.
                    // e.g., forward_to_validator_consensus(deposit);
                } else {
                    println!("[XRPL Witness] ❌ Transaction was not a valid deposit.");
                }
            }
            Ok(_) => {} // Other stream messages
            Err(e) => eprintln!("[XRPL Witness] Error in stream: {:?}", e),
        }
    }
}

/// Parses an XRPL transaction to check if it's a valid deposit.
fn parse_deposit_transaction(tx: &xrpl_rust::models::transactions::Transaction) -> Option<DepositInfo> {
    // We are looking for a simple Payment transaction.
    if let xrpl_rust::models::transactions::Transaction::Payment(payment) = tx {
        // 1. Must be sent TO our door account.
        if payment.common.account.to_string() != DOOR_ACCOUNT {
            return None;
        }

        // 2. Must have a DestinationTag, which we use as the L2 identifier.
        if payment.destination_tag.is_none() {
            return None;
        }
        
        // 3. Amount must be XRP.
        if let xrpl_rust::models::Amount::Xrp(amount) = &payment.amount {
            return Some(DepositInfo {
                from: payment.common.account.to_string(),
                amount: amount.to_string(), // Amount in drops (1 million drops = 1 XRP)
                l2_destination: payment.destination_tag.unwrap(),
            });
        }
    }
    None
}
