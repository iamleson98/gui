//! Question #458: JIT vs Interpreter Performance
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: JIT, interpreter, compilation, warmup
//! Description: Compare JIT compilation with interpretation and where each pays off.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct JitVsInterpreterPerformance {
    inner: Mutex<HashMap<String, String>>,
}

impl JitVsInterpreterPerformance {
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
    fn test_jit_vs_interpreter_performance() {
        let s = JitVsInterpreterPerformance::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
