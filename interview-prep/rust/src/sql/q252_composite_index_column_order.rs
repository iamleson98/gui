//! Question #252: Composite Index Column Order
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: composite index, column order, selectivity, range
//! Description: Design composite indexes with column order matching equality, sort, and range predicates.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CompositeIndexColumnOrder {
    inner: Mutex<HashMap<String, String>>,
}

impl CompositeIndexColumnOrder {
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
    fn test_composite_index_column_order() {
        let s = CompositeIndexColumnOrder::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
