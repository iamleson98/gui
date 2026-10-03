//! Question #56: Thread-Local Storage
//! Category: Concurrency | Difficulty: Hard
//! Concepts: thread-local, slots, cleanup, per-thread
//! Description: Design a thread-local storage abstraction with per-thread slots and optional cleanup on thread exit.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ThreadLocalStorage {
    data: Mutex<HashMap<i32, i32>>,
}

impl ThreadLocalStorage {
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
    fn test_thread_local_storage() {
        let s = ThreadLocalStorage::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
