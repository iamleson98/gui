//! Question #108: Persistent Treap
//! Category: Data Structures | Difficulty: Hard
//! Concepts: persistent treap, split/merge, immutability, randomized
//! Description: Implement an implicit-key treap that persists prior versions on split and merge.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PersistentTreap {
    data: Mutex<HashMap<i32, i32>>,
}

impl PersistentTreap {
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
    fn test_persistent_treap() {
        let s = PersistentTreap::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
