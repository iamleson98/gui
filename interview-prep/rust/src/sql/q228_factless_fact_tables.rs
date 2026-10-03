//! Question #228: Factless Fact Tables
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: factless fact, coverage, many-to-many, event
//! Description: Model many-to-many event coverage with factless fact tables capturing only keys.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FactlessFactTables {
    inner: Mutex<HashMap<String, String>>,
}

impl FactlessFactTables {
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
    fn test_factless_fact_tables() {
        let s = FactlessFactTables::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
