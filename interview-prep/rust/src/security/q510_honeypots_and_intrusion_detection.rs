//! Question #510: Honeypots and Intrusion Detection
//! Category: Security | Difficulty: Hard
//! Concepts: honeypot, IDS, detection, deception
//! Description: Deploy honeypots and IDS to detect and analyze attackers.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HoneypotsAndIntrusionDetection {
    inner: Mutex<HashMap<String, String>>,
}

impl HoneypotsAndIntrusionDetection {
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
    fn test_honeypots_and_intrusion_detection() {
        let s = HoneypotsAndIntrusionDetection::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
