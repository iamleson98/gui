//! Question #369: tcmalloc Design
//! Category: Memory Management | Difficulty: Hard
//! Concepts: tcmalloc, thread-local, spans, central heap
//! Description: Explain tcmalloc's thread-local caches and span-based central heap for scalable allocation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TcmallocDesign {
    inner: Mutex<HashMap<String, String>>,
}

impl TcmallocDesign {
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
    fn test_tcmalloc_design() {
        let s = TcmallocDesign::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
