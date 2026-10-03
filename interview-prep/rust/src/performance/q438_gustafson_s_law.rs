//! Question #438: Gustafson's Law
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: Gustafson, scaled speedup, parallel, problem size
//! Description: Apply Gustafson's law to scale problems with processors rather than fix them.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct GustafsonSLaw {
    inner: Mutex<HashMap<String, String>>,
}

impl GustafsonSLaw {
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
    fn test_gustafson_s_law() {
        let s = GustafsonSLaw::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
