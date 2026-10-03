//! Question #535: DNS over HTTPS/TLS
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: DoH, DoT, encryption, privacy
//! Description: Encrypt DNS queries with DoH/DoT and reason about privacy and policy implications.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DnsOverHttpsTls {
    inner: Mutex<HashMap<String, String>>,
}

impl DnsOverHttpsTls {
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
    fn test_dns_over_https_tls() {
        let s = DnsOverHttpsTls::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
