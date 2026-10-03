//! Question #453: Lock Contention Profiling
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: lock contention, profiling, sharding, lock-free
//! Description: Profile lock contention to find contended locks and convert them to sharded or lock-free forms.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LockContentionProfiling {
    inner: Mutex<HashMap<String, String>>,
}

impl LockContentionProfiling {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
    pub fn set(&self, key: &str, val: &str) {
        self.inner.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lock_contention_profiling() {
        let s = LockContentionProfiling::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
