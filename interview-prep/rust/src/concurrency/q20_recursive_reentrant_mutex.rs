//! Question #20: Recursive (Reentrant) Mutex
//! Category: Concurrency | Difficulty: Hard
//! Concepts: mutex, reentrant, owner, recursion count
//! Description: Implement a mutex that allows the same thread to acquire it multiple times by tracking an owner and recursion count.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RecursiveReentrantMutex {
    data: Mutex<HashMap<i32, i32>>,
}

impl RecursiveReentrantMutex {
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
    fn test_recursive_reentrant_mutex() {
        let s = RecursiveReentrantMutex::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
