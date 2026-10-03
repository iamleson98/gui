//! Question #308: Design an Event Ticketing System
//! Category: System Design | Difficulty: Hard
//! Concepts: flash sale, virtual queue, inventory, contention
//! Description: Design high-contention flash-sale ticketing with virtual queues and inventory sharding.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnEventTicketingSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnEventTicketingSystem {
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
    fn test_design_an_event_ticketing_system() {
        let s = DesignAnEventTicketingSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
