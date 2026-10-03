//! Question #415: strace / ltrace
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: strace, ltrace, syscall, library
//! Description: Use strace and ltrace to attribute time spent in system and library calls.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct StraceLtrace {
    inner: Mutex<HashMap<String, String>>,
}

impl StraceLtrace {
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
    fn test_strace_ltrace() {
        let s = StraceLtrace::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
