//! Question #280: Outbox Pattern
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: outbox, exactly-once, event publishing, transactional
//! Description: Reliably publish events to a broker by writing them transactionally to an outbox table.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OutboxPattern {
    inner: Mutex<HashMap<String, String>>,
}

impl OutboxPattern {
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
    fn test_outbox_pattern() {
        let s = OutboxPattern::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
