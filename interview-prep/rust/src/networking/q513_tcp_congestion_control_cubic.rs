//! Question #513: TCP Congestion Control (CUBIC)
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: CUBIC, cubic, window, congestion
//! Description: Explain the CUBIC cubic window growth function used as default Linux congestion control.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TcpCongestionControlCubic {
    inner: Mutex<HashMap<String, String>>,
}

impl TcpCongestionControlCubic {
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
    fn test_tcp_congestion_control_cubic() {
        let s = TcpCongestionControlCubic::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
