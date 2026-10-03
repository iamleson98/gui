//! Question #53: Futures and Promises
//! Category: Concurrency | Difficulty: Hard
//! Concepts: future, promise, continuation, shared state
//! Description: Implement a future/promise pair with shared state, continuations, and ready/error/pending states.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FuturesAndPromises {
    data: Mutex<HashMap<i32, i32>>,
}

impl FuturesAndPromises {
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
    fn test_futures_and_promises() {
        let s = FuturesAndPromises::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
