//! Question #300: Design a Presence Service
//! Category: System Design | Difficulty: Hard
//! Concepts: presence, heartbeat, pub/sub, fan-out
//! Description: Design a presence service tracking online status with heartbeats and a pub/sub fan-out.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAPresenceService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAPresenceService {
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
    fn test_design_a_presence_service() {
        let s = DesignAPresenceService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
