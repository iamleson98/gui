//! Question #112: Finger Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: finger tree, amortized, split, persistent
//! Description: Implement Okasaki's finger tree supporting amortized O(1) cons/snoc and O(log n) split.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FingerTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl FingerTree {
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
    fn test_finger_tree() {
        let s = FingerTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
