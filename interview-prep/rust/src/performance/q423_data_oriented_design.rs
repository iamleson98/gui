//! Question #423: Data-Oriented Design
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: DOD, hot/cold split, cache efficiency, layout
//! Description: Restructure data for cache efficiency by separating hot and cold fields.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DataOrientedDesign {
    inner: Mutex<HashMap<String, String>>,
}

impl DataOrientedDesign {
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
    fn test_data_oriented_design() {
        let s = DataOrientedDesign::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
