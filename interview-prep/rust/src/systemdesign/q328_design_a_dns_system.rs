//! Question #328: Design a DNS System
//! Category: System Design | Difficulty: Hard
//! Concepts: DNS, hierarchy, TTL, negative caching
//! Description: Design a hierarchical, cached DNS resolver with TTLs and negative caching.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADnsSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADnsSystem {
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
    fn test_design_a_dns_system() {
        let s = DesignADnsSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
