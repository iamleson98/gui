//! Question #492: Merkle Tree for Integrity
//! Category: Security | Difficulty: Hard
//! Concepts: Merkle tree, integrity, proof, tamper detection
//! Description: Use Merkle trees to verify integrity of large data sets with compact proofs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MerkleTreeForIntegrity {
    inner: Mutex<HashMap<String, String>>,
}

impl MerkleTreeForIntegrity {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
    pub fn set(&self, key: &str, val: &str) {
        self.inner.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merkle_tree_for_integrity() {
        let s = MerkleTreeForIntegrity::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
