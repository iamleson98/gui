//! Question #293: Design Twitter Timeline Fan-Out
//! Category: System Design | Difficulty: Hard
//! Concepts: fan-out, push, pull, celebrity
//! Description: Compare push-on-write vs pull-on-read fan-out for celebrity and normal users.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignTwitterTimelineFanOut {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignTwitterTimelineFanOut {
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
    fn test_design_twitter_timeline_fan_out() {
        let s = DesignTwitterTimelineFanOut::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
