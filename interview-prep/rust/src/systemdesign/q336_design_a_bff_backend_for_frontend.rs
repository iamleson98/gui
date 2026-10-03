//! Question #336: Design a BFF (Backend for Frontend)
//! Category: System Design | Difficulty: Hard
//! Concepts: BFF, aggregation, client-specific, edge
//! Description: Design a backend-for-frontend layer that aggregates services for a specific client.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignABffBackendForFrontend {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignABffBackendForFrontend {
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
    fn test_design_a_bff_backend_for_frontend() {
        let s = DesignABffBackendForFrontend::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
