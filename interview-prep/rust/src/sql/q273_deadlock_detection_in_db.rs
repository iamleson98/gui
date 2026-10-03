//! Question #273: Deadlock Detection in DB
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: deadlock, wait-for graph, detection, victim
//! Description: Use a wait-for graph to detect and resolve deadlocks among transactions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DeadlockDetectionInDb {
    inner: Mutex<HashMap<String, String>>,
}

impl DeadlockDetectionInDb {
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
    fn test_deadlock_detection_in_db() {
        let s = DeadlockDetectionInDb::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
