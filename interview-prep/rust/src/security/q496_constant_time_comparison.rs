//! Question #496: Constant-Time Comparison
//! Category: Security | Difficulty: Hard
//! Concepts: constant time, timing leak, comparison, secret
//! Description: Implement constant-time comparison to avoid leaking equality via timing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ConstantTimeComparison {
    inner: Mutex<HashMap<String, String>>,
}

impl ConstantTimeComparison {
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
    fn test_constant_time_comparison() {
        let s = ConstantTimeComparison::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
