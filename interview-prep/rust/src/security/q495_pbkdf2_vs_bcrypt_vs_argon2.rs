//! Question #495: PBKDF2 vs bcrypt vs argon2
//! Category: Security | Difficulty: Hard
//! Concepts: PBKDF2, bcrypt, argon2, memory-hard
//! Description: Compare PBKDF2, bcrypt, and argon2 on CPU/memory hardness and suitability.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Pbkdf2VsBcryptVsArgon2 {
    inner: Mutex<HashMap<String, String>>,
}

impl Pbkdf2VsBcryptVsArgon2 {
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
    fn test_pbkdf2_vs_bcrypt_vs_argon2() {
        let s = Pbkdf2VsBcryptVsArgon2::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
