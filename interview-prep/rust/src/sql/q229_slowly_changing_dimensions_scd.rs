//! Question #229: Slowly Changing Dimensions (SCD)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: SCD, Type 2, history, effective dates
//! Description: Implement SCD Types 1-4 to track history of dimension attribute changes over time.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SlowlyChangingDimensionsScd {
    inner: Mutex<HashMap<String, String>>,
}

impl SlowlyChangingDimensionsScd {
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
    fn test_slowly_changing_dimensions_scd() {
        let s = SlowlyChangingDimensionsScd::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
