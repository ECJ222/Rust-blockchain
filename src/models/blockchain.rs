use super::block::Block;
use chrono::prelude::*;

type Blocks = Vec<Block>;

// `Blockchain` A struct that represents the blockchain.
#[derive(Debug, Clone)]
pub struct Blockchain {
    // The first block to be added to the chain.
    pub genesis_block: Block,
    // The storage for blocks.
    pub chain: Blocks,
    // Minimum amount of work required to mine a block.
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // First block in the chain.
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };

        // Create chain starting from the genesis chain.
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        // Create a blockchain Instance.
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty,
        };

        blockchain
    }

    pub fn add_block(&mut self) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone(),
        );

        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block);
    }
}
