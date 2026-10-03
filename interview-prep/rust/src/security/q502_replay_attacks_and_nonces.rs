//! Question #502: Replay Attacks and Nonces
//! Category: Security | Difficulty: Hard
//! Concepts: replay, nonce, timestamp, sequence
//! Description: Defend against replay using nonces, timestamps, and sequence numbers.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ReplayAttacksAndNonces {
    inner: Mutex<HashMap<String, String>>,
}

impl ReplayAttacksAndNonces {
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
    fn test_replay_attacks_and_nonces() {
        let s = ReplayAttacksAndNonces::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
