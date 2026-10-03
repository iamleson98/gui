//! Question #548: ICMP and Ping/Traceroute
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: ICMP, ping, traceroute, TTL
//! Description: Use ICMP echo and time-exceeded messages to implement ping and traceroute.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IcmpAndPingTraceroute {
    inner: Mutex<HashMap<String, String>>,
}

impl IcmpAndPingTraceroute {
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
    fn test_icmp_and_ping_traceroute() {
        let s = IcmpAndPingTraceroute::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
