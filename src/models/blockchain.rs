use chrono::prelude::*;
use super::block::Block;

type Blocks = Vec<Block>;

// `Blockchain` A struct that represents the blockchain.
#[derive(Debug)]
pub struct Blockchain {
    // The first block to be added to the chain.
    pub genesis_block: Block,
    // The storage for blocks.
    pub chain: Blocks,
    // Minimum effort to validate a block.
    pub difficulty: usize
}

impl Blockchain {
    pub fn new() -> Self {
        // First block in the chain.
        let mut genesis_block = Block {
            index: 0,
            nonce: String::from("0"),
            timestamp: Utc::now().timestamp_millis() as u64,
            previous_hash: String::default(),
            hash: String::from("0")
        };

        genesis_block.hash = genesis_block.calculate_hash();

        // Create chain starting from the genesis chain.
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        // Create a blockchain Instance.
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty: 3
        };

        blockchain
    }

    pub fn add_block(&mut self, nonce: String) {
        let new_block = Block::new(
            self.chain.len() as u64,
            nonce,
            self.chain[&self.chain.len() - 1].previous_hash.clone()
        );

        // Verify block before it's added.
        if Blockchain::verify_block(self, new_block.clone()) {
            self.chain.push(new_block.clone());
            println!("New block added to chain -> {:?}", &self);
        } else {
            println!("Block is not valid -> {:?}", &self);
        }
    }

    pub fn verify_block (&mut self, block: Block) -> bool {
        let pattern = "0".repeat(self.difficulty);
        let mut hash = pattern;
        let mut nonce_hash = block.nonce;

        hash.push_str(block.hash.as_str());
        nonce_hash.push_str(block.hash.as_str());


        if nonce_hash == hash {
            self.difficulty += 1;
            return true
        } else {
            return false
        }
    }
}
