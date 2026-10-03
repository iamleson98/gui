//! Question #331: Design a Layer-7 Load Balancer
//! Category: System Design | Difficulty: Hard
//! Concepts: L7 LB, content routing, TLS termination, retries
//! Description: Design an L7 load balancer with content-based routing, TLS termination, and retries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignALayer7LoadBalancer {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignALayer7LoadBalancer {
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
    fn test_design_a_layer_7_load_balancer() {
        let s = DesignALayer7LoadBalancer::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
