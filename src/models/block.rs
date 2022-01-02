use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // The index in which the current block is stored.
    pub index: u64,
    // An abitiary number used for verification.
    pub nonce: String,
    // The time the current block is created.
    pub timestamp: u64,
    // The previous block hash.
    pub previous_hash: String,
    // The current block hash.
    pub hash: String
}

impl Block {
    // Create a new block. The hash will be calculated and set automatically.
    pub fn new (
        index: u64,
        nonce: String,
        previous_hash: String,
    ) -> Self {
        // Current block to be created.
        let mut block = Block {
            index,
            nonce: nonce,
            timestamp: Utc::now().timestamp_millis() as u64,
            previous_hash,
            hash: String::from("0"),
        };

        block.hash = block.calculate_hash();

        block
    }

    // Calculate hash value of the new block.
    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        // Convert block to JSON format.
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        // Calculate and return SHA-256 hash value.
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);

        let result = hasher.finalize();

        format!("{:x}", result)
    }
}
