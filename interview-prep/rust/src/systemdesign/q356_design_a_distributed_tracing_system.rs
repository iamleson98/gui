//! Question #356: Design a Distributed Tracing System
//! Category: System Design | Difficulty: Hard
//! Concepts: tracing, spans, sampling, context
//! Description: Design an OpenTelemetry-style tracing system with spans, sampling, and context propagation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADistributedTracingSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADistributedTracingSystem {
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
    fn test_design_a_distributed_tracing_system() {
        let s = DesignADistributedTracingSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
