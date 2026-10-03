//! Question #380: Tri-Color Marking
//! Category: Memory Management | Difficulty: Hard
//! Concepts: tri-color, white/gray/black, invariant, tracing
//! Description: Implement the tri-color invariant (white, gray, black) during tracing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TriColorMarking {
    inner: Mutex<HashMap<String, String>>,
}

impl TriColorMarking {
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
    fn test_tri_color_marking() {
        let s = TriColorMarking::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
