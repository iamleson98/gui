//! Question #418: CPU Pipeline Stalls
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: pipeline, stall, data hazard, latency
//! Description: Identify pipeline stalls from data hazards and long-latency instructions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CpuPipelineStalls {
    inner: Mutex<HashMap<String, String>>,
}

impl CpuPipelineStalls {
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
    fn test_cpu_pipeline_stalls() {
        let s = CpuPipelineStalls::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
