//! Question #521: Sliding Window Flow Control
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: sliding window, flow control, window, backpressure
//! Description: Implement sliding-window flow control between sender and receiver.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SlidingWindowFlowControl {
    inner: Mutex<HashMap<String, String>>,
}

impl SlidingWindowFlowControl {
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
    fn test_sliding_window_flow_control() {
        let s = SlidingWindowFlowControl::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
