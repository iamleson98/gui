//! Question #416: gprof and the GNU Profiler
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: gprof, call graph, flat profile, instrumentation
//! Description: Use gprof call-graph and flat profiles to locate hot functions in C programs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct GprofAndTheGnuProfiler {
    inner: Mutex<HashMap<String, String>>,
}

impl GprofAndTheGnuProfiler {
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
    fn test_gprof_and_the_gnu_profiler() {
        let s = GprofAndTheGnuProfiler::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
