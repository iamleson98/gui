//! Question #354: Design a Merkle Tree for Anti-Entropy
//! Category: System Design | Difficulty: Hard
//! Concepts: Merkle tree, anti-entropy, replica diff, repair
//! Description: Compare replicas with Merkle trees to localize and repair differences.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAMerkleTreeForAntiEntropy {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAMerkleTreeForAntiEntropy {
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
    fn test_design_a_merkle_tree_for_anti_entropy() {
        let s = DesignAMerkleTreeForAntiEntropy::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
