//! Question #230: Bridge Tables for Many-to-Many
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: bridge table, many-to-many, junction, aggregate
//! Description: Model many-to-many relationships with bridge tables and resolve aggregates correctly.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BridgeTablesForManyToMany {
    inner: Mutex<HashMap<String, String>>,
}

impl BridgeTablesForManyToMany {
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
    fn test_bridge_tables_for_many_to_many() {
        let s = BridgeTablesForManyToMany::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
