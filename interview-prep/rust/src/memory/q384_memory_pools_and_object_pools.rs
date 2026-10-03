//! Question #384: Memory Pools and Object Pools
//! Category: Memory Management | Difficulty: Hard
//! Concepts: object pool, amortize, construct cost, reuse
//! Description: Design object pools to amortize allocation of expensive-to-construct objects.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryPoolsAndObjectPools {
    inner: Mutex<HashMap<String, String>>,
}

impl MemoryPoolsAndObjectPools {
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
    fn test_memory_pools_and_object_pools() {
        let s = MemoryPoolsAndObjectPools::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
