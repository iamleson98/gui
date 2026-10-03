//! Question #409: NUMA Allocation Policies
//! Category: Memory Management | Difficulty: Hard
//! Concepts: NUMA, first-touch, locality, allocation
//! Description: Apply NUMA-aware allocation and first-touch to keep memory local to compute.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct NumaAllocationPolicies {
    inner: Mutex<HashMap<String, String>>,
}

impl NumaAllocationPolicies {
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
    fn test_numa_allocation_policies() {
        let s = NumaAllocationPolicies::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
