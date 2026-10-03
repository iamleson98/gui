//! Question #224: Denormalization Tradeoffs
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: denormalization, read performance, anomalies, tradeoff
//! Description: Evaluate when denormalizing for read performance outweighs the cost of update anomalies.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DenormalizationTradeoffs {
    inner: Mutex<HashMap<String, String>>,
}

impl DenormalizationTradeoffs {
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
    fn test_denormalization_tradeoffs() {
        let s = DenormalizationTradeoffs::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
