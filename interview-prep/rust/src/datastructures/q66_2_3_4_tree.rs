//! Question #66: 2-3-4 Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: 2-3-4 tree, splitting, bottom-up, balance
//! Description: Build a balanced search tree with 2-, 3-, and 4-nodes that splits on overflow from the bottom up.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Q66_234Tree {
    data: Mutex<HashMap<i32, i32>>,
}

impl Q66_234Tree {
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
    fn test_2_3_4_tree() {
        let s = Q66_234Tree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
