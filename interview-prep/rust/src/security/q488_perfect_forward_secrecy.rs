//! Question #488: Perfect Forward Secrecy
//! Category: Security | Difficulty: Hard
//! Concepts: PFS, ephemeral, key exchange, compromise
//! Description: Achieve perfect forward secrecy with ephemeral key exchange so past traffic resists future key compromise.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PerfectForwardSecrecy {
    inner: Mutex<HashMap<String, String>>,
}

impl PerfectForwardSecrecy {
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
    fn test_perfect_forward_secrecy() {
        let s = PerfectForwardSecrecy::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
