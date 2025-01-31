use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = &self.chain[self.chain.len() - 1];
        let new_block = Block::new(prev_block.index + 1, data, prev_block.hash.clone());
        self.chain.push(new_block);
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.prev_hash != previous.hash {
                return false;
            }

            let recalculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.prev_hash,
            );

            if current.hash != recalculated_hash {
                return false;
            }
        }
        true
    }
}
