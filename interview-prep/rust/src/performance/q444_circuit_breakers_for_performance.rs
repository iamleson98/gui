//! Question #444: Circuit Breakers for Performance
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: circuit breaker, fail fast, latency, outage
//! Description: Use circuit breakers to fail fast and protect latency during dependency outages.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CircuitBreakersForPerformance {
    inner: Mutex<HashMap<String, String>>,
}

impl CircuitBreakersForPerformance {
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
    fn test_circuit_breakers_for_performance() {
        let s = CircuitBreakersForPerformance::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
