//! Question #392: TLB and TLB Shootdown
//! Category: Memory Management | Difficulty: Hard
//! Concepts: TLB, shootdown, IPI, translation cache
//! Description: Explain the TLB cache of translations and the cost of cross-CPU shootdowns.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TlbAndTlbShootdown {
    inner: Mutex<HashMap<String, String>>,
}

impl TlbAndTlbShootdown {
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
    fn test_tlb_and_tlb_shootdown() {
        let s = TlbAndTlbShootdown::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
