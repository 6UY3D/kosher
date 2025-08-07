use crate::config::WitnessConfig;
use xrpl_rust::client::{Client, JsonRpcClient};
use xrpl_rust::models::requests::{LedgerRequest, Subscribe};
use xrpl_rust::models::streams::StreamMessage;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

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

type PendingQueue = Arc<Mutex<Vec<PendingDeposit>>>;

pub async fn run_xrpl_witness(config: WitnessConfig) {
    println!("[XRPL Witness] Starting service...");
    let client = match JsonRpcClient::new(config.xrpl_node_url.parse().unwrap()) {
        Ok(c) => Arc::new(c),
        Err(e) => {
            eprintln!("[XRPL Witness] Failed to create client: {}", e);
            return;
        }
    };
    
    let pending_queue: PendingQueue = Arc::new(Mutex::new(Vec::new()));

    let listener_client = Arc::clone(&client);
    let listener_queue = Arc::clone(&pending_queue);
    let door_account_clone = config.door_account.clone();
    tokio::spawn(listen_for_deposits(listener_client, listener_queue, door_account_clone));

    let processor_client = Arc::clone(&client);
    let processor_queue = Arc::clone(&pending_queue);
    tokio::spawn(process_confirmed_deposits(processor_client, processor_queue, config.confirmation_threshold));
}

async fn listen_for_deposits(client: Arc<JsonRpcClient>, queue: PendingQueue, door_account: String) {
    let sub = Subscribe::new().accounts(vec![door_account.clone()]).build().unwrap();
    let mut stream = match client.subscribe(&sub).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[XRPL Witness] Failed to subscribe: {}", e);
            return;
        }
    };
    println!("[XRPL Witness] Subscribed to door account: {}", door_account);

    while let Some(msg) = stream.next().await {
        if let Ok(StreamMessage::Transaction(tx)) = msg {
            if let Some(info) = parse_deposit_transaction(&tx, &door_account) {
                let ledger_index = tx.common().ledger_index.unwrap_or(0);
                println!("[XRPL Witness] Saw potential deposit in ledger {}. Adding to pending queue.", ledger_index);
                let deposit = PendingDeposit { info, seen_in_ledger: ledger_index };
                queue.lock().unwrap().push(deposit);
            }
        }
    }
}

async fn process_confirmed_deposits(client: Arc<JsonRpcClient>, queue: PendingQueue, confirmation_threshold: u32) {
    loop {
        sleep(Duration::from_secs(15)).await;

        let current_ledger_index = match client.ledger(LedgerRequest::current()).await {
            Ok(resp) => resp.ledger_index,
            Err(_) => continue,
        };

        let mut deposits_to_process = Vec::new();
        let mut queue_lock = queue.lock().unwrap();

        queue_lock.retain(|deposit| {
            if (current_ledger_index.saturating_sub(deposit.seen_in_ledger)) >= confirmation_threshold {
                deposits_to_process.push(deposit.clone());
                false
            } else {
                true
            }
        });

        drop(queue_lock);

        for deposit in deposits_to_process {
            println!("[XRPL Witness] ✅ CONFIRMED deposit from ledger {}: {:?}", deposit.seen_in_ledger, deposit.info);
            // Here, the witness would trigger the L2 consensus mechanism to mint tokens.
        }
    }
}

fn parse_deposit_transaction(tx: &xrpl_rust::models::transactions::Transaction, door_account: &str) -> Option<DepositInfo> {
    if let xrpl_rust::models::transactions::Transaction::Payment(payment) = tx {
        if payment.destination.to_string() == door_account {
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
