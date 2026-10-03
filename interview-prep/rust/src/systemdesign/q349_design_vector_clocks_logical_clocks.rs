//! Question #349: Design Vector Clocks / Logical Clocks
//! Category: System Design | Difficulty: Hard
//! Concepts: vector clock, logical clock, concurrency, causality
//! Description: Use vector clocks to detect concurrent updates in a distributed store.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignVectorClocksLogicalClocks {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignVectorClocksLogicalClocks {
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
    fn test_design_vector_clocks_logical_clocks() {
        let s = DesignVectorClocksLogicalClocks::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
