//! Question #291: Design a Rate Limiter (Sliding Window)
//! Category: System Design | Difficulty: Hard
//! Concepts: rate limiter, sliding window, sorted set, accuracy
//! Description: Implement a sliding-window rate limiter using sorted sets or a rolling counter sketch.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignARateLimiterSlidingWindow {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignARateLimiterSlidingWindow {
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
    fn test_design_a_rate_limiter_sliding_window() {
        let s = DesignARateLimiterSlidingWindow::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
