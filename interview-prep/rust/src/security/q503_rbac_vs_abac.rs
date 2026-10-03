//! Question #503: RBAC vs ABAC
//! Category: Security | Difficulty: Hard
//! Concepts: RBAC, ABAC, authorization, policy
//! Description: Choose between role-based and attribute-based access control for authorization.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RbacVsAbac {
    inner: Mutex<HashMap<String, String>>,
}

impl RbacVsAbac {
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
    fn test_rbac_vs_abac() {
        let s = RbacVsAbac::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
