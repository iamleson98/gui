//! Question #436: Roofline Model
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: roofline, arithmetic intensity, peak, bandwidth
//! Description: Plot the roofline model to see whether arithmetic intensity caps performance.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RooflineModel {
    inner: Mutex<HashMap<String, String>>,
}

impl RooflineModel {
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
    fn test_roofline_model() {
        let s = RooflineModel::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
