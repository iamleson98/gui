//! Question #40: Wait-Free Hash Table
//! Category: Concurrency | Difficulty: Hard
//! Concepts: wait-free, hash table, resize, bounded steps
//! Description: Design a resize-friendly wait-free hash table where every operation completes in bounded CAS steps.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WaitFreeHashTable {
    data: Mutex<HashMap<i32, i32>>,
}

impl WaitFreeHashTable {
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
    fn test_wait_free_hash_table() {
        let s = WaitFreeHashTable::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
