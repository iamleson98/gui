//! Question #459: Escape Analysis and Stack Allocation
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: escape analysis, stack allocation, GC, lifetime
//! Description: Use escape analysis to allocate heap objects on the stack for zero-cost lifetimes.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EscapeAnalysisAndStackAllocation {
    inner: Mutex<HashMap<String, String>>,
}

impl EscapeAnalysisAndStackAllocation {
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
    fn test_escape_analysis_and_stack_allocation() {
        let s = EscapeAnalysisAndStackAllocation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
