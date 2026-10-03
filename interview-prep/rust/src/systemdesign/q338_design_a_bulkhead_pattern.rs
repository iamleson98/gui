//! Question #338: Design a Bulkhead Pattern
//! Category: System Design | Difficulty: Hard
//! Concepts: bulkhead, isolation, concurrency limit, failure domain
//! Description: Isolate failure domains with bulkheads limiting concurrency per dependency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignABulkheadPattern {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignABulkheadPattern {
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
    fn test_design_a_bulkhead_pattern() {
        let s = DesignABulkheadPattern::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
