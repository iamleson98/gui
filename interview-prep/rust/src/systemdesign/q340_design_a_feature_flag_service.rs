//! Question #340: Design a Feature Flag Service
//! Category: System Design | Difficulty: Hard
//! Concepts: feature flags, targeting, live config, rollouts
//! Description: Design a feature flag service with targeting rules and live config updates.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAFeatureFlagService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAFeatureFlagService {
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
    fn test_design_a_feature_flag_service() {
        let s = DesignAFeatureFlagService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
