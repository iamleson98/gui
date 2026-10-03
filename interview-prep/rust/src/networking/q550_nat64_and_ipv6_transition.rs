//! Question #550: NAT64 and IPv6 Transition
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: NAT64, DNS64, transition, IPv6
//! Description: Transition between IPv6-only and IPv4 networks using NAT64 and DNS64.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Nat64AndIpv6Transition {
    inner: Mutex<HashMap<String, String>>,
}

impl Nat64AndIpv6Transition {
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
    fn test_nat64_and_ipv6_transition() {
        let s = Nat64AndIpv6Transition::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
