//! Question #301: Design a Notification Service
//! Category: System Design | Difficulty: Hard
//! Concepts: notifications, fan-out, dedup, preferences
//! Description: Design a multi-channel notification fan-out with dedup, batching, and user preferences.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignANotificationService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignANotificationService {
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
    fn test_design_a_notification_service() {
        let s = DesignANotificationService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
