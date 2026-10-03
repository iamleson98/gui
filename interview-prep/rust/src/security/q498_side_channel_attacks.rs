//! Question #498: Side-Channel Attacks
//! Category: Security | Difficulty: Hard
//! Concepts: side channel, cache, power, EM
//! Description: Defend against cache, power, and EM side channels in sensitive code.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SideChannelAttacks {
    inner: Mutex<HashMap<String, String>>,
}

impl SideChannelAttacks {
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
    fn test_side_channel_attacks() {
        let s = SideChannelAttacks::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
