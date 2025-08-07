use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use std::convert::TryInto;

// A wallet holds the cryptographic keypair needed to sign data.
pub struct Wallet {
    keypair: SigningKey,
}

impl Wallet {
    /// Creates a new wallet with a fresh keypair.
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = SigningKey::generate(&mut csprng);
        Self { keypair }
    }

    /// Signs a given message (e.g., a block hash).
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    /// Returns the public key as a hex string for identification.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.keypair.verifying_key().to_bytes())
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
        let public_key = VerifyingKey::from_bytes(&pubkey_array).expect("Failed to create public key");
        
        public_key.verify_strict(message, signature).is_ok()
    }
}    let mut rogue_block = Block {
        header: BlockHeader {
            id: previous_block.header.id + 1,
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash: previous_block.calculate_header_hash(),
            validator_pubkey: unauthorized_wallet.public_key_hex(),
            transactions_hash: Block::hash_transactions(&rogue_transactions),
        },
        transactions: rogue_transactions,
        signature: Signature::from_bytes(&[0; 64]).unwrap(),
    };
    let rogue_hash = rogue_block.calculate_header_hash();
    rogue_block.signature = unauthorized_wallet.sign(rogue_hash.as_bytes());

    match kosher_chain.add_block(rogue_block) {
        Ok(_) => println!("✅ SUCCESS: Block added by rogue actor."),
        Err(e) => eprintln!("❌ FAILURE: {}. Block was correctly rejected.", e),
    }

    println!("\nFinal block count: {}", kosher_chain.blocks.len());

    // Save the updated chain
    kosher_chain.save_to_file(CHAIN_FILE).unwrap();
}
