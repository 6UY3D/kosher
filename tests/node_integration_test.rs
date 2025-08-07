use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use kosher_chain::block::Transaction; // Assuming your crate is named kosher_chain

#[test]
#[ignore] // Ignored by default as it's a long-running test
fn test_node_receives_transaction_and_creates_block() {
    // 1. Set up a temporary directory for all test files
    let temp = TempDir::new().unwrap();
    let config_path = temp.child("config.toml");
    let validators_path = temp.child("validators.json");
    let key_path = temp.child("validator_key.json");
    let state_path = temp.child("blockchain_state.json");

    // 2. Create the validator key
    // In a real test, we would run a command or use the wallet code to generate this.
    // For now, we assume it's pre-generated and its public key is known.
    let validator_pubkey = "your_known_validator_public_key_hex";

    // 3. Create config files
    validators_path.write_str(&format!(r#"{{"validators": ["{}"]}}"#, validator_pubkey)).unwrap();
    config_path.write_str(&format!(r#"
        [api]
        listen_address = "127.0.0.1:8080"
        [p2p]
        listen_address = "/ip4/0.0.0.0/tcp/0"
        [witness]
        xrpl_node_url = "wss://s1.ripple.com:51234"
        door_account = "r..."
        confirmation_threshold = 10
        [chain]
        validators_file = "{}"
        [validator]
        key_file = "{}"
    "#, validators_path.path().to_str().unwrap(), key_path.path().to_str().unwrap())).unwrap();

    // 4. Run the node binary in the background
    let mut node_process = Command::new("cargo")
        .args(&["run", "--release"])
        .current_dir(temp.path()) // Run from the temp directory
        .spawn()
        .expect("Failed to start node process");
        
    sleep(Duration::from_secs(5)); // Wait for the node to start up

    // 5. Send a transaction to the API
    let client = reqwest::blocking::Client::new();
    let sample_tx: Transaction = // create a valid, signed transaction here
    
    let res = client.post("http://127.0.0.1:8080/transaction")
        .json(&sample_tx)
        .send()
        .unwrap();
    assert!(res.status().is_success());

    // 6. Wait for the validator to propose a block
    sleep(Duration::from_secs(20));

    // 7. Shut down the node and verify the state
    node_process.kill().unwrap();
    
    let state_content = state_path.read_to_string().unwrap();
    assert!(state_content.contains(&sample_tx.hash));
}
