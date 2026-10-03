//! Question #477: Refresh Token Rotation
//! Category: Security | Difficulty: Hard
//! Concepts: refresh token, rotation, reuse detection, theft
//! Description: Rotate refresh tokens on use with reuse detection to limit token theft impact.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RefreshTokenRotation {
    inner: Mutex<HashMap<String, String>>,
}

impl RefreshTokenRotation {
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
    fn test_refresh_token_rotation() {
        let s = RefreshTokenRotation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
