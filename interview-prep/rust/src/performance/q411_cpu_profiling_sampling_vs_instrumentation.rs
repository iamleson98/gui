//! Question #411: CPU Profiling: Sampling vs Instrumentation
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: profiling, sampling, instrumentation, overhead
//! Description: Contrast sampling and instrumentation profilers for accuracy vs overhead.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CpuProfilingSamplingVsInstrumentation {
    inner: Mutex<HashMap<String, String>>,
}

impl CpuProfilingSamplingVsInstrumentation {
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
    fn test_cpu_profiling_sampling_vs_instrumentation() {
        let s = CpuProfilingSamplingVsInstrumentation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
