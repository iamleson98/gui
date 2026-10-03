//! Question #327: Design an Email Service
//! Category: System Design | Difficulty: Hard
//! Concepts: email, provider failover, bounce, throttling
//! Description: Design a transactional email service with provider failover and bounce handling.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnEmailService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnEmailService {
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
    fn test_design_an_email_service() {
        let s = DesignAnEmailService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
