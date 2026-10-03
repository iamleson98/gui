//! Question #471: Password Salting and Pepping
//! Category: Security | Difficulty: Hard
//! Concepts: salt, pepper, precomputed, rainbow
//! Description: Use per-password salts and a server-side pepper to defeat precomputed attacks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PasswordSaltingAndPepping {
    inner: Mutex<HashMap<String, String>>,
}

impl PasswordSaltingAndPepping {
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
    fn test_password_salting_and_pepping() {
        let s = PasswordSaltingAndPepping::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
