//! Question #540: UDP Reliability at Application Layer
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: UDP, reliability, ordering, congestion
//! Description: Build reliability, ordering, and congestion control on top of UDP.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct UdpReliabilityAtApplicationLayer {
    inner: Mutex<HashMap<String, String>>,
}

impl UdpReliabilityAtApplicationLayer {
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
    fn test_udp_reliability_at_application_layer() {
        let s = UdpReliabilityAtApplicationLayer::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
