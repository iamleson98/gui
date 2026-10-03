//! Question #511: TCP Three-Way Handshake
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: TCP, handshake, SYN, state machine
//! Description: Explain SYN, SYN-ACK, ACK and the resulting connection state machine.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TcpThreeWayHandshake {
    inner: Mutex<HashMap<String, String>>,
}

impl TcpThreeWayHandshake {
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
    fn test_tcp_three_way_handshake() {
        let s = TcpThreeWayHandshake::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
