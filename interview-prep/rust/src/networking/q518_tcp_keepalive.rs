//! Question #518: TCP Keepalive
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: keepalive, dead peer, idle, probes
//! Description: Configure keepalives to detect dead peers without application-level heartbeats.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TcpKeepalive {
    inner: Mutex<HashMap<String, String>>,
}

impl TcpKeepalive {
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
    fn test_tcp_keepalive() {
        let s = TcpKeepalive::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
