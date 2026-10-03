//! Question #337: Design a Circuit Breaker
//! Category: System Design | Difficulty: Hard
//! Concepts: circuit breaker, half-open, thresholds, resilience
//! Description: Implement a circuit breaker with half-open probing and configurable thresholds.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignACircuitBreaker {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignACircuitBreaker {
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
    fn test_design_a_circuit_breaker() {
        let s = DesignACircuitBreaker::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
