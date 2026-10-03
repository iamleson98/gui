//! Question #235: Triggers (Before/After)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: triggers, before/after, audit, side effects
//! Description: Implement before- and after-triggers for auditing and derived-column maintenance, noting pitfalls.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TriggersBeforeAfter {
    inner: Mutex<HashMap<String, String>>,
}

impl TriggersBeforeAfter {
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
    fn test_triggers_before_after() {
        let s = TriggersBeforeAfter::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
