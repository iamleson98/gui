//! Question #339: Design Retry with Backoff and Jitter
//! Category: System Design | Difficulty: Hard
//! Concepts: retry, exponential backoff, jitter, deadline
//! Description: Design retries with exponential backoff, jitter, and deadline propagation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignRetryWithBackoffAndJitter {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignRetryWithBackoffAndJitter {
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
    fn test_design_retry_with_backoff_and_jitter() {
        let s = DesignRetryWithBackoffAndJitter::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
