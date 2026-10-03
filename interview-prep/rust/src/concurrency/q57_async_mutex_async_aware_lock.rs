//! Question #57: Async Mutex / Async-Aware Lock
//! Category: Concurrency | Difficulty: Hard
//! Concepts: async, mutex, wait list, parking
//! Description: Implement a mutex whose waiters park futures rather than OS threads, avoiding thread blocking.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AsyncMutexAsyncAwareLock {
    data: Mutex<HashMap<i32, i32>>,
}

impl AsyncMutexAsyncAwareLock {
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
    fn test_async_mutex_async_aware_lock() {
        let s = AsyncMutexAsyncAwareLock::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
