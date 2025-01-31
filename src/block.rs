use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &data, &prev_hash);
        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: i64, data: &str, prev_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}
