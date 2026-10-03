//! Question #17: Eventfd / Event Signaling
//! Category: Concurrency | Difficulty: Hard
//! Concepts: eventfd, signaling, counter, edge-trigger
//! Description: Build an eventfd-like counter used for cross-thread signaling with overflow protection and level/edge semantics.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EventfdEventSignaling {
    data: Mutex<HashMap<i32, i32>>,
}

impl EventfdEventSignaling {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
    pub fn insert(&self, key: i32, val: i32) {
        self.data.lock().unwrap().insert(key, val);
    }
    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.lock().unwrap().get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eventfd_event_signaling() {
        let s = EventfdEventSignaling::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
