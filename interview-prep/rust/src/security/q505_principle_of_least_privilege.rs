//! Question #505: Principle of Least Privilege
//! Category: Security | Difficulty: Hard
//! Concepts: least privilege, scoping, minimization, principle
//! Description: Apply least privilege by granting the minimum scopes needed for a task.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PrincipleOfLeastPrivilege {
    inner: Mutex<HashMap<String, String>>,
}

impl PrincipleOfLeastPrivilege {
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
    fn test_principle_of_least_privilege() {
        let s = PrincipleOfLeastPrivilege::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
