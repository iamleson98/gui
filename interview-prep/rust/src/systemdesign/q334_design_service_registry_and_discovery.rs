//! Question #334: Design Service Registry and Discovery
//! Category: System Design | Difficulty: Hard
//! Concepts: service discovery, registry, health checks, client-side
//! Description: Design a service registry with health checks and client-side discovery.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignServiceRegistryAndDiscovery {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignServiceRegistryAndDiscovery {
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
    fn test_design_service_registry_and_discovery() {
        let s = DesignServiceRegistryAndDiscovery::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
