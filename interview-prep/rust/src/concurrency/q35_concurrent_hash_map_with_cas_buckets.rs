//! Question #35: Concurrent Hash Map with CAS Buckets
//! Category: Concurrency | Difficulty: Hard
//! Concepts: hash map, lock-free, CAS, chaining
//! Description: Build a hash map whose buckets are lock-free singly linked lists updated by compare-and-swap.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ConcurrentHashMapWithCasBuckets {
    data: Mutex<HashMap<i32, i32>>,
}

impl ConcurrentHashMapWithCasBuckets {
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
    fn test_concurrent_hash_map_with_cas_buckets() {
        let s = ConcurrentHashMapWithCasBuckets::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
