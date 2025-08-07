use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature, KeypairBytes};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
use crate::errors::NodeError;

/// A persistent wallet that can be saved to and loaded from disk.
pub struct Wallet {
    keypair: SigningKey,
}

impl Wallet {
    /// Creates a new, random wallet.
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = SigningKey::generate(&mut csprng);
        Self { keypair }
    }

    /// Signs a given message with the wallet's private key.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    /// Returns the public key as a hex string for identification.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.keypair.verifying_key().to_bytes())
    }
    
    /// Loads a wallet from a keypair file. If the file doesn't exist,
    /// it creates a new wallet and saves it to that path.
    pub fn load_or_create(path: &Path) -> Result<Self, NodeError> {
        if path.exists() {
            let keypair_bytes: KeypairBytes = serde_json::from_str(
                &fs::read_to_string(path)?
            ).map_err(|e| NodeError::Config(format!("Failed to parse keypair file: {}", e)))?;
            
            let keypair = SigningKey::from_bytes(&keypair_bytes.secret_key);

            println!("[Wallet] Loaded existing wallet from {}", path.display());
            Ok(Self { keypair })
        } else {
            println!("[Wallet] No wallet found. Creating new wallet at {}", path.display());
            let wallet = Self::new();
            wallet.save(path)?;
            Ok(wallet)
        }
    }
    
    /// Saves the wallet's keypair to a file.
    pub fn save(&self, path: &Path) -> Result<(), NodeError> {
        let keypair_bytes = self.keypair.to_keypair_bytes();
        let json_bytes = serde_json::to_string_pretty(&keypair_bytes)?;
        fs::write(path, json_bytes)?;
        Ok(())
    }

    /// Verifies a signature given a public key, the original message, and the signature.
    pub fn verify_signature(public_key_hex: &str, message: &[u8], signature: &Signature) -> bool {
        let pubkey_bytes = match hex::decode(public_key_hex) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let pubkey_array: [u8; 32] = match pubkey_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };
        let public_key = match VerifyingKey::from_bytes(&pubkey_array) {
            Ok(key) => key,
            Err(_) => return false,
        };
        
        public_key.verify_strict(message, signature).is_ok()
    }
}
