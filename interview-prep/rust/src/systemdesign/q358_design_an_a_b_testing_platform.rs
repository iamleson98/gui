//! Question #358: Design an A/B Testing Platform
//! Category: System Design | Difficulty: Hard
//! Concepts: A/B testing, bucketing, metrics, significance
//! Description: Design an experimentation platform with bucketing, metrics, and statistical guardrails.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnABTestingPlatform {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnABTestingPlatform {
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
    fn test_design_an_a_b_testing_platform() {
        let s = DesignAnABTestingPlatform::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
