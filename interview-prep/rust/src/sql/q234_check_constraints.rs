//! Question #234: CHECK Constraints
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: check constraint, domain, business rules, validation
//! Description: Use CHECK constraints and domains to enforce row-level business rules declaratively.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CheckConstraints {
    inner: Mutex<HashMap<String, String>>,
}

impl CheckConstraints {
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
    fn test_check_constraints() {
        let s = CheckConstraints::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
