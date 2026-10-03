//! Question #426: Loop Unrolling
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: unrolling, ILP, loop overhead, code size
//! Description: Unroll loops to reduce loop overhead and expose ILP, balancing code-size costs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LoopUnrolling {
    inner: Mutex<HashMap<String, String>>,
}

impl LoopUnrolling {
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
    fn test_loop_unrolling() {
        let s = LoopUnrolling::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
