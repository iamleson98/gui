//! Question #538: BGP Basics
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: BGP, AS path, interdomain, routing
//! Description: Explain BGP path selection and how AS-path attributes drive interdomain routing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BgpBasics {
    inner: Mutex<HashMap<String, String>>,
}

impl BgpBasics {
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
    fn test_bgp_basics() {
        let s = BgpBasics::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
