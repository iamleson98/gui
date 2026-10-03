//! Question #472: JWT Design and Pitfalls
//! Category: Security | Difficulty: Hard
//! Concepts: JWT, alg, expiry, signature
//! Description: Design JWTs securely, avoiding alg=none, weak keys, and missing expiry validation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct JwtDesignAndPitfalls {
    inner: Mutex<HashMap<String, String>>,
}

impl JwtDesignAndPitfalls {
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
    fn test_jwt_design_and_pitfalls() {
        let s = JwtDesignAndPitfalls::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
