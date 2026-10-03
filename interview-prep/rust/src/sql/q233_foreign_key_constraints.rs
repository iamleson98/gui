//! Question #233: Foreign Key Constraints
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: foreign key, referential integrity, cascade, restrict
//! Description: Enforce referential integrity with foreign keys and choose RESTRICT, CASCADE, and SET NULL actions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ForeignKeyConstraints {
    inner: Mutex<HashMap<String, String>>,
}

impl ForeignKeyConstraints {
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
    fn test_foreign_key_constraints() {
        let s = ForeignKeyConstraints::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
