use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Blockchain operation failed: {0}")]
    Blockchain(String),

    #[error("P2P networking error: {0}")]
    P2p(String),

    #[error("API server error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Persistence error: {0}")]
    Persistence(#[from] std::io::Error),

    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("XRPL Witness error: {0}")]
    Witness(String),
}
