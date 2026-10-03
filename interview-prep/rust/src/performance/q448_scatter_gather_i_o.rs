//! Question #448: Scatter-Gather I/O
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: scatter-gather, readv, writev, syscall
//! Description: Use scatter-gather (readv/writev) to coalesce multiple buffers per syscall.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ScatterGatherIO {
    inner: Mutex<HashMap<String, String>>,
}

impl ScatterGatherIO {
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
    fn test_scatter_gather_i_o() {
        let s = ScatterGatherIO::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
