//! Question #541: WebRTC and ICE/STUN/TURN
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: WebRTC, ICE, STUN, TURN
//! Description: Establish peer-to-peer media with ICE candidate gathering and TURN fallback.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WebrtcAndIceStunTurn {
    inner: Mutex<HashMap<String, String>>,
}

impl WebrtcAndIceStunTurn {
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
    fn test_webrtc_and_ice_stun_turn() {
        let s = WebrtcAndIceStunTurn::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
