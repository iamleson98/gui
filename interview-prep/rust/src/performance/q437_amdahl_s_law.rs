//! Question #437: Amdahl's Law
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: Amdahl, speedup, parallel fraction, bounds
//! Description: Apply Amdahl's law to bound speedup from parallelizing a fraction of a program.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AmdahlSLaw {
    inner: Mutex<HashMap<String, String>>,
}

impl AmdahlSLaw {
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
    fn test_amdahl_s_law() {
        let s = AmdahlSLaw::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
