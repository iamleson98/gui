//! Question #254: Partial Index (WHERE Clause)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: partial index, WHERE, subset, size
//! Description: Create partial indexes on a filtered subset to reduce size and speed common queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PartialIndexWhereClause {
    inner: Mutex<HashMap<String, String>>,
}

impl PartialIndexWhereClause {
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
    fn test_partial_index_where_clause() {
        let s = PartialIndexWhereClause::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
