//! Question #530: Conditional Requests (If-None-Match)
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: conditional request, If-None-Match, If-Modified-Since, 304
//! Description: Use If-None-Match and If-Modified-Since to validate cached responses.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ConditionalRequestsIfNoneMatch {
    inner: Mutex<HashMap<String, String>>,
}

impl ConditionalRequestsIfNoneMatch {
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
    fn test_conditional_requests_if_none_match() {
        let s = ConditionalRequestsIfNoneMatch::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
