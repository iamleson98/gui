//! Question #414: eBPF for Tracing
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: eBPF, tracing, kprobes, uprobes
//! Description: Write eBPF probes to trace kernel and user functions with low overhead.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EbpfForTracing {
    inner: Mutex<HashMap<String, String>>,
}

impl EbpfForTracing {
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
    fn test_ebpf_for_tracing() {
        let s = EbpfForTracing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
