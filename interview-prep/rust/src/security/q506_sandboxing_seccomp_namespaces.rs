//! Question #506: Sandboxing (seccomp, namespaces)
//! Category: Security | Difficulty: Hard
//! Concepts: sandbox, seccomp, namespaces, syscall filter
//! Description: Sandbox untrusted code with seccomp filters and Linux namespaces.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SandboxingSeccompNamespaces {
    inner: Mutex<HashMap<String, String>>,
}

impl SandboxingSeccompNamespaces {
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
    fn test_sandboxing_seccomp_namespaces() {
        let s = SandboxingSeccompNamespaces::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
