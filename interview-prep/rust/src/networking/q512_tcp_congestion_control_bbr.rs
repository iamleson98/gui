//! Question #512: TCP Congestion Control (BBR)
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: BBR, congestion, bandwidth, RTT
//! Description: Explain BBR's model-based congestion control versus loss-based schemes.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TcpCongestionControlBbr {
    inner: Mutex<HashMap<String, String>>,
}

impl TcpCongestionControlBbr {
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
    fn test_tcp_congestion_control_bbr() {
        let s = TcpCongestionControlBbr::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
