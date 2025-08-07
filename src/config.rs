use serde::Deserialize;
use std::fs;
use crate::errors::NodeError;

#[derive(Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub p2p: P2pConfig,
    pub witness: WitnessConfig,
    pub chain: ChainConfig,
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub listen_address: String,
}

#[derive(Deserialize)]
pub struct P2pConfig {
    pub listen_address: String,
    // pub bootstrap_nodes: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct WitnessConfig {
    pub xrpl_node_url: String,
    pub door_account: String,
    pub confirmation_threshold: u32,
}

#[derive(Deserialize)]
pub struct ChainConfig {
    pub validators_file: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, NodeError> {
        let content = fs::read_to_string(path)
            .map_err(|e| NodeError::Config(format!("Failed to read config file {}: {}", path, e)))?;
            
        toml::from_str(&content)
            .map_err(|e| NodeError::Config(format!("Failed to parse config file: {}", e)))
    }
}
