//! Question #343: Design Multi-Region Active-Active
//! Category: System Design | Difficulty: Hard
//! Concepts: active-active, multi-region, conflict, routing
//! Description: Design an active-active multi-region system handling conflict resolution and routing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignMultiRegionActiveActive {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignMultiRegionActiveActive {
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
    fn test_design_multi_region_active_active() {
        let s = DesignMultiRegionActiveActive::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
