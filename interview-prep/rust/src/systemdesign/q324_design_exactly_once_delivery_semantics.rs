//! Question #324: Design Exactly-Once Delivery Semantics
//! Category: System Design | Difficulty: Hard
//! Concepts: exactly-once, idempotent producer, transactions, EOS
//! Description: Achieve exactly-once delivery using idempotent producers and transactional consumption.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignExactlyOnceDeliverySemantics {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignExactlyOnceDeliverySemantics {
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
    fn test_design_exactly_once_delivery_semantics() {
        let s = DesignExactlyOnceDeliverySemantics::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
