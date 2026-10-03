//! Question #542: WebSocket Protocol
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: WebSocket, upgrade, framing, ping/pong
//! Description: Upgrade to WebSocket for bidirectional, low-latency messaging with framing and ping/pong.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WebsocketProtocol {
    inner: Mutex<HashMap<String, String>>,
}

impl WebsocketProtocol {
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
    fn test_websocket_protocol() {
        let s = WebsocketProtocol::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
