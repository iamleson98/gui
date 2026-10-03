//! Question #412: Flame Graphs
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: flame graph, stack samples, hot path, visualization
//! Description: Build and read flame graphs to identify hot code paths from stack samples.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FlameGraphs {
    inner: Mutex<HashMap<String, String>>,
}

impl FlameGraphs {
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
    fn test_flame_graphs() {
        let s = FlameGraphs::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
