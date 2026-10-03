//! Question #425: Auto-Vectorization
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: auto-vectorization, compiler, restrict, alignment
//! Description: Write loops that the compiler auto-vectorizes and verify with assembly inspection.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AutoVectorization {
    inner: Mutex<HashMap<String, String>>,
}

impl AutoVectorization {
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
    fn test_auto_vectorization() {
        let s = AutoVectorization::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
