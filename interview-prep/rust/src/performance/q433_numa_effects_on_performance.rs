//! Question #433: NUMA Effects on Performance
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: NUMA, remote access, locality, sockets
//! Description: Diagnose NUMA-local vs remote access penalties for multi-socket workloads.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct NumaEffectsOnPerformance {
    inner: Mutex<HashMap<String, String>>,
}

impl NumaEffectsOnPerformance {
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
    fn test_numa_effects_on_performance() {
        let s = NumaEffectsOnPerformance::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
