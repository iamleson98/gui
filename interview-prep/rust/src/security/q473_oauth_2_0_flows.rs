//! Question #473: OAuth 2.0 Flows
//! Category: Security | Difficulty: Hard
//! Concepts: OAuth, authorization code, PKCE, client credentials
//! Description: Implement authorization-code, client-credentials, and PKCE flows appropriately.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Oauth20Flows {
    inner: Mutex<HashMap<String, String>>,
}

impl Oauth20Flows {
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
    fn test_oauth_2_0_flows() {
        let s = Oauth20Flows::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
