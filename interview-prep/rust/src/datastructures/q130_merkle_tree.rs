//! Question #130: Merkle Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Merkle tree, hash, inclusion proof, tamper detection
//! Description: Implement a Merkle tree of content hashes supporting inclusion proofs and tamper detection.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MerkleTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
    pub fn insert(&self, key: i32, val: i32) {
        self.data.lock().unwrap().insert(key, val);
    }
    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.lock().unwrap().get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merkle_tree() {
        let s = MerkleTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
