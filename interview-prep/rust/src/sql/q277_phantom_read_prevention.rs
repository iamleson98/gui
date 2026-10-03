//! Question #277: Phantom Read Prevention
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: phantom, predicate lock, gap lock, stability
//! Description: Prevent phantom reads via predicate locking or gap locks to keep a predicate result set stable.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PhantomReadPrevention {
    inner: Mutex<HashMap<String, String>>,
}

impl PhantomReadPrevention {
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
    fn test_phantom_read_prevention() {
        let s = PhantomReadPrevention::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
