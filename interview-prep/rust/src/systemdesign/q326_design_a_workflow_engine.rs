//! Question #326: Design a Workflow Engine
//! Category: System Design | Difficulty: Hard
//! Concepts: workflow, durable, retries, timers
//! Description: Design a durable workflow engine (Temporal/Airflow-like) with retries, timers, and state.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAWorkflowEngine {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAWorkflowEngine {
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
    fn test_design_a_workflow_engine() {
        let s = DesignAWorkflowEngine::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
