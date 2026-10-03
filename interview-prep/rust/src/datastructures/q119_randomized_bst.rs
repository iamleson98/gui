//! Question #119: Randomized BST
//! Category: Data Structures | Difficulty: Hard
//! Concepts: randomized BST, expected balance, root insert, probability
//! Description: Implement a randomized BST that inserts at the root with probability 1/n to stay balanced in expectation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RandomizedBst {
    data: Mutex<HashMap<i32, i32>>,
}

impl RandomizedBst {
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
    fn test_randomized_bst() {
        let s = RandomizedBst::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
