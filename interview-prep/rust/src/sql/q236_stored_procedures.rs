//! Question #236: Stored Procedures
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: stored procedures, encapsulation, security, maintainability
//! Description: Design stored procedures for encapsulated logic and weigh security and maintainability tradeoffs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct StoredProcedures {
    inner: Mutex<HashMap<String, String>>,
}

impl StoredProcedures {
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
    fn test_stored_procedures() {
        let s = StoredProcedures::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
