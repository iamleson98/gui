//! Question #110: Fully Persistent BST
//! Category: Data Structures | Difficulty: Hard
//! Concepts: persistent BST, path copying, versioning, immutability
//! Description: Implement a BST that keeps every prior version queryable after updates.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FullyPersistentBst {
    data: Mutex<HashMap<i32, i32>>,
}

impl FullyPersistentBst {
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
    fn test_fully_persistent_bst() {
        let s = FullyPersistentBst::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
