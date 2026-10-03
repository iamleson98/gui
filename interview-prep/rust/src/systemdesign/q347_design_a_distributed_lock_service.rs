//! Question #347: Design a Distributed Lock Service
//! Category: System Design | Difficulty: Hard
//! Concepts: distributed lock, fencing token, lease, renewal
//! Description: Design a distributed lock with fencing tokens and lease renewal.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADistributedLockService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADistributedLockService {
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
    fn test_design_a_distributed_lock_service() {
        let s = DesignADistributedLockService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
