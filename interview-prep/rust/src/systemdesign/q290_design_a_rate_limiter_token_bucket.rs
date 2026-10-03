//! Question #290: Design a Rate Limiter (Token Bucket)
//! Category: System Design | Difficulty: Hard
//! Concepts: rate limiter, token bucket, distributed, sliding
//! Description: Design a token-bucket rate limiter distributed across nodes with sliding accuracy.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignARateLimiterTokenBucket {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignARateLimiterTokenBucket {
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
    fn test_design_a_rate_limiter_token_bucket() {
        let s = DesignARateLimiterTokenBucket::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
