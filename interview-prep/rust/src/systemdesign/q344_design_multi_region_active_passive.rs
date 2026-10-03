//! Question #344: Design Multi-Region Active-Passive
//! Category: System Design | Difficulty: Hard
//! Concepts: active-passive, failover, replication, RTO
//! Description: Design an active-passive multi-region system with failover and data replication.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignMultiRegionActivePassive {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignMultiRegionActivePassive {
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
    fn test_design_multi_region_active_passive() {
        let s = DesignMultiRegionActivePassive::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
