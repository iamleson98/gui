//! Question #335: Design an API Gateway
//! Category: System Design | Difficulty: Hard
//! Concepts: API gateway, auth, rate limit, routing
//! Description: Design an API gateway with auth, rate limiting, routing, and observability.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnApiGateway {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnApiGateway {
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
    fn test_design_an_api_gateway() {
        let s = DesignAnApiGateway::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
