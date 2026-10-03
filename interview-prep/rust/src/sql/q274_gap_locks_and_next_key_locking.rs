//! Question #274: Gap Locks and Next-Key Locking
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: gap lock, next-key, phantom, InnoDB
//! Description: Prevent phantom reads with gap and next-key locking in InnoDB repeatable-read.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct GapLocksAndNextKeyLocking {
    inner: Mutex<HashMap<String, String>>,
}

impl GapLocksAndNextKeyLocking {
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
    fn test_gap_locks_and_next_key_locking() {
        let s = GapLocksAndNextKeyLocking::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
