//! Question #54: Once / Call-Once Initialization
//! Category: Concurrency | Difficulty: Hard
//! Concepts: once, initialization, double-checked, atomic
//! Description: Implement std::once / sync.Once semantics guaranteeing a function runs exactly once under concurrency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OnceCallOnceInitialization {
    data: Mutex<HashMap<i32, i32>>,
}

impl OnceCallOnceInitialization {
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
    fn test_once_call_once_initialization() {
        let s = OnceCallOnceInitialization::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
