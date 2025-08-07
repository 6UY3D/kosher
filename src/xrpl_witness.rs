use xrpl_rust::client::{Client, JsonRpcClient};
use xrpl_rust::models::requests::{LedgerRequest, Subscribe};
use xrpl_rust::models::streams::StreamMessage;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

// The public address of the multisignature door account on the XRPL.
const DOOR_ACCOUNT: &str = "rDoorAccountAddressOnXrpl...";

// The number of ledgers we wait for before considering a transaction final.
// While XRPL has fast finality, a buffer is a crucial security practice.
const CONFIRMATION_THRESHOLD: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInfo {
    pub from: String,       
    pub amount: String,     
    pub l2_destination: u64,
}

#[derive(Debug, Clone)]
struct PendingDeposit {
    info: DepositInfo,
    seen_in_ledger: u32,
}

// A thread-safe queue for deposits awaiting confirmation.
type PendingQueue = Arc<Mutex<Vec<PendingDeposit>>>;

/// Main entry point for the witness service.
/// Spawns two concurrent tasks: one to listen for new transactions and one to process confirmed transactions.
pub async fn run_xrpl_witness() {
    println!("[XRPL Witness] Starting service...");
    let client = Arc::new(JsonRpcClient::new("wss://s1.ripple.com:51234".parse().unwrap()).unwrap());
    let pending_queue: PendingQueue = Arc::new(Mutex::new(Vec::new()));

    // Task 1: Listen for new transactions and add them to the pending queue.
    let listener_client = Arc::clone(&client);
    let listener_queue = Arc::clone(&pending_queue);
    tokio::spawn(listen_for_deposits(listener_client, listener_queue));

    // Task 2: Periodically check the pending queue and process confirmed deposits.
    let processor_client = Arc::clone(&client);
    let processor_queue = Arc::clone(&pending_queue);
    tokio::spawn(process_confirmed_deposits(processor_client, processor_queue));
}

/// Listens to the XRPL stream and adds potential deposits to a pending queue.
async fn listen_for_deposits(client: Arc<JsonRpcClient>, queue: PendingQueue) {
    let sub = Subscribe::new().accounts(vec![DOOR_ACCOUNT.to_string()]).build().unwrap();
    let mut stream = client.subscribe(&sub).await.expect("Failed to subscribe");
    println!("[XRPL Witness] Subscribed to door account: {}", DOOR_ACCOUNT);

    while let Some(msg) = stream.next().await {
        if let Ok(StreamMessage::Transaction(tx)) = msg {
            if let Some(info) = parse_deposit_transaction(&tx) {
                let ledger_index = tx.common().ledger_index.unwrap_or(0);
                println!("[XRPL Witness] Saw potential deposit in ledger {}. Adding to pending queue.", ledger_index);
                let deposit = PendingDeposit { info, seen_in_ledger: ledger_index };
                queue.lock().unwrap().push(deposit);
            }
        }
    }
}

/// Periodically processes deposits from the queue that have met the confirmation threshold.
async fn process_confirmed_deposits(client: Arc<JsonRpcClient>, queue: PendingQueue) {
    loop {
        sleep(Duration::from_secs(15)).await; // Check every 15 seconds

        let current_ledger_index = match client.ledger(LedgerRequest::current()).await {
            Ok(resp) => resp.ledger_index,
            Err(_) => {
                eprintln!("[XRPL Witness] Could not fetch current ledger index.");
                continue;
            }
        };

        let mut deposits_to_process = Vec::new();
        let mut queue_lock = queue.lock().unwrap();

        // Drain the queue, moving confirmed deposits to a temporary vector.
        // This avoids holding the lock while processing.
        queue_lock.retain(|deposit| {
            if (current_ledger_index - deposit.seen_in_ledger) >= CONFIRMATION_THRESHOLD {
                deposits_to_process.push(deposit.clone());
                false // Remove from queue
            } else {
                true // Keep in queue
            }
        });

        drop(queue_lock); // Release the lock

        for deposit in deposits_to_process {
            println!("[XRPL Witness] ✅ CONFIRMED deposit from ledger {}. Processing now.", deposit.seen_in_ledger);
            println!("[XRPL Witness]    Details: {:?}", deposit.info);
            // In a real system, this is where the witness would broadcast a signed vote
            // to the other validators to trigger the minting of kXRP on the L2.
            // e.g., trigger_l2_mint_consensus(deposit.info);
        }
    }
}


/// Parses an XRPL transaction to check if it's a valid deposit. (Unchanged)
fn parse_deposit_transaction(tx: &xrpl_rust::models::transactions::Transaction) -> Option<DepositInfo> {
    if let xrpl_rust::models::transactions::Transaction::Payment(payment) = tx {
        if payment.destination.to_string() == DOOR_ACCOUNT {
            if let (Some(dest_tag), Some(sender)) = (payment.destination_tag, &payment.common.account) {
                if let xrpl_rust::models::Amount::Xrp(amount) = &payment.amount {
                    return Some(DepositInfo {
                        from: sender.to_string(),
                        amount: amount.to_string(),
                        l2_destination: dest_tag,
                    });
                }
            }
        }
    }
    None
}
