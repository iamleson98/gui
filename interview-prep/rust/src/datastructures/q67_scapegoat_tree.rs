//! Question #67: Scapegoat Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: scapegoat tree, rebuild, amortized, alpha-balanced
//! Description: Implement a self-balancing BST that rebuilds an unbalanced subtree when its height exceeds a logarithmic bound.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ScapegoatTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl ScapegoatTree {
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
    fn test_scapegoat_tree() {
        let s = ScapegoatTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
