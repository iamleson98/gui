//! Question #507: Container Security
//! Category: Security | Difficulty: Hard
//! Concepts: container, capabilities, read-only, image
//! Description: Harden containers with reduced capabilities, read-only roots, and minimal images.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ContainerSecurity {
    inner: Mutex<HashMap<String, String>>,
}

impl ContainerSecurity {
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
    fn test_container_security() {
        let s = ContainerSecurity::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
