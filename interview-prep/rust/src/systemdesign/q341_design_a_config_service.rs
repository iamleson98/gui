//! Question #341: Design a Config Service
//! Category: System Design | Difficulty: Hard
//! Concepts: config, dynamic, watch, versioning
//! Description: Design a dynamic configuration service with watch notifications and versioning.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAConfigService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAConfigService {
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
    fn test_design_a_config_service() {
        let s = DesignAConfigService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
