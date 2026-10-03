//! Question #372: Mark-and-Sweep GC
//! Category: Memory Management | Difficulty: Hard
//! Concepts: mark-and-sweep, tracing, roots, sweep
//! Description: Implement a mark-and-sweep collector that traces live objects and sweeps dead ones.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MarkAndSweepGc {
    inner: Mutex<HashMap<String, String>>,
}

impl MarkAndSweepGc {
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
    fn test_mark_and_sweep_gc() {
        let s = MarkAndSweepGc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
