//! Question #402: Overcommit and OOM
//! Category: Memory Management | Difficulty: Hard
//! Concepts: overcommit, commit limit, OOM, accounting
//! Description: Reason about memory overcommit, commit limits, and the consequences for OOM.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OvercommitAndOom {
    inner: Mutex<HashMap<String, String>>,
}

impl OvercommitAndOom {
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
    fn test_overcommit_and_oom() {
        let s = OvercommitAndOom::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
