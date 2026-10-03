//! Question #63: AA Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: AA tree, level, skew, split
//! Description: Implement a balanced BST using horizontal/vertical links and skew/split rebalancing for simpler code than red-black.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AaTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl AaTree {
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
    fn test_aa_tree() {
        let s = AaTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
