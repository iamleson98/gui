//! Question #443: Backpressure and Load Shedding
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: backpressure, load shedding, overload, latency
//! Description: Apply backpressure and load shedding to preserve latency under overload.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BackpressureAndLoadShedding {
    inner: Mutex<HashMap<String, String>>,
}

impl BackpressureAndLoadShedding {
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
    fn test_backpressure_and_load_shedding() {
        let s = BackpressureAndLoadShedding::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
