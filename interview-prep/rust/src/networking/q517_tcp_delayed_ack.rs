//! Question #517: TCP Delayed ACK
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: delayed ACK, Nagle, latency, writes
//! Description: Explain delayed ACK and the latency it can introduce with small writes.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TcpDelayedAck {
    inner: Mutex<HashMap<String, String>>,
}

impl TcpDelayedAck {
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
    fn test_tcp_delayed_ack() {
        let s = TcpDelayedAck::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
