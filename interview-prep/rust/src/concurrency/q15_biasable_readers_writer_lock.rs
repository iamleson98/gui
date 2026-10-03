//! Question #15: Biasable Readers-Writer Lock
//! Category: Concurrency | Difficulty: Hard
//! Concepts: readers-writers, biasing, throughput, fairness
//! Description: Design an RW lock that can be biased toward readers or writers and rebiased at runtime to tune throughput.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BiasableReadersWriterLock {
    data: Mutex<HashMap<i32, i32>>,
}

impl BiasableReadersWriterLock {
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
    fn test_biasable_readers_writer_lock() {
        let s = BiasableReadersWriterLock::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
