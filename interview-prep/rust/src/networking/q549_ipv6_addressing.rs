//! Question #549: IPv6 Addressing
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: IPv6, subnetting, SLAAC, addressing
//! Description: Explain IPv6 address structure, subnetting, and stateless autoconfiguration.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Ipv6Addressing {
    inner: Mutex<HashMap<String, String>>,
}

impl Ipv6Addressing {
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
    fn test_ipv6_addressing() {
        let s = Ipv6Addressing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
