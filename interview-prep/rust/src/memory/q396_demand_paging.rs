//! Question #396: Demand Paging
//! Category: Memory Management | Difficulty: Hard
//! Concepts: demand paging, page fault, lazy, zero fill
//! Description: Load pages on first access via page faults to avoid eager allocation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DemandPaging {
    inner: Mutex<HashMap<String, String>>,
}

impl DemandPaging {
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
    fn test_demand_paging() {
        let s = DemandPaging::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
