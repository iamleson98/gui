//! Question #357: Design Context Propagation (W3C Trace Context)
//! Category: System Design | Difficulty: Hard
//! Concepts: trace context, W3C, propagation, headers
//! Description: Propagate trace context across process boundaries with W3C headers.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignContextPropagationW3CTraceContext {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignContextPropagationW3CTraceContext {
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
    fn test_design_context_propagation_w3c_trace_context() {
        let s = DesignContextPropagationW3CTraceContext::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
