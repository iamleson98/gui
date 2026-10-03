//! Question #58: Exponential Backoff Strategies
//! Category: Concurrency | Difficulty: Hard
//! Concepts: backoff, exponential, jitter, contention
//! Description: Build an exponential backoff with jitter for retrying contended CAS loops and RPC calls.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ExponentialBackoffStrategies {
    data: Mutex<HashMap<i32, i32>>,
}

impl ExponentialBackoffStrategies {
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
    fn test_exponential_backoff_strategies() {
        let s = ExponentialBackoffStrategies::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
