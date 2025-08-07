use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature, KeypairBytes};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
use crate::errors::NodeError;

pub struct Wallet {
    keypair: SigningKey,
}

// ... new(), sign(), public_key_hex(), load_or_create(), save(), verify_signature() functions remain the same ...

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    #[test]
    fn test_wallet_create_sign_verify() {
        let wallet = Wallet::new();
        let message = b"test message";
        let signature = wallet.sign(message);
        
        let pubkey_hex = wallet.public_key_hex();
        assert!(Wallet::verify_signature(&pubkey_hex, message, &signature));
    }

    #[test]
    fn test_wallet_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let key_file_path = temp_dir.child("test_wallet.json");

        // Create and save a new wallet
        let wallet1 = Wallet::new();
        let pubkey1 = wallet1.public_key_hex();
        wallet1.save(key_file_path.path()).unwrap();

        // Load the wallet back from the file
        let wallet2 = Wallet::load_or_create(key_file_path.path()).unwrap();
        let pubkey2 = wallet2.public_key_hex();

        assert_eq!(pubkey1, pubkey2);

        // Verify that the loaded wallet can still sign messages correctly
        let message = b"another message";
        let signature = wallet2.sign(message);
        assert!(Wallet::verify_signature(&pubkey2, message, &signature));
    }
}            Ok(arr) => arr,
            Err(_) => return false,
        };
        let public_key = match VerifyingKey::from_bytes(&pubkey_array) {
            Ok(key) => key,
            Err(_) => return false,
        };
        
        public_key.verify_strict(message, signature).is_ok()
    }
}
