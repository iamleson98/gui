//! Question #310: Design an Inventory Service
//! Category: System Design | Difficulty: Hard
//! Concepts: inventory, reservation, strong consistency, SKU
//! Description: Design a per-SKU inventory service with strong consistency and reservation semantics.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnInventoryService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnInventoryService {
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
    fn test_design_an_inventory_service() {
        let s = DesignAnInventoryService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
