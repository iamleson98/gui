//! Question #342: Design a Secret Management Service
//! Category: System Design | Difficulty: Hard
//! Concepts: secrets, encryption, leasing, audit
//! Description: Design a Vault-like secret store with encryption, leasing, and audit logging.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignASecretManagementService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignASecretManagementService {
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
    fn test_design_a_secret_management_service() {
        let s = DesignASecretManagementService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
