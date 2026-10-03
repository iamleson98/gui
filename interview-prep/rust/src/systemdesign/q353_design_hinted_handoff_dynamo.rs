//! Question #353: Design Hinted Handoff (Dynamo)
//! Category: System Design | Difficulty: Hard
//! Concepts: hinted handoff, Dynamo, replica, recovery
//! Description: Store writes for temporarily unavailable replicas and hand them off on recovery.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignHintedHandoffDynamo {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignHintedHandoffDynamo {
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
    fn test_design_hinted_handoff_dynamo() {
        let s = DesignHintedHandoffDynamo::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
