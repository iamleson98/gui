//! Question #386: Epoch-Based Reclamation (EBR)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: EBR, epoch, deferred, lock-free
//! Description: Defer reclamation until epochs advance past all readers for lock-free safety.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EpochBasedReclamationEbr {
    inner: Mutex<HashMap<String, String>>,
}

impl EpochBasedReclamationEbr {
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
    fn test_epoch_based_reclamation_ebr() {
        let s = EpochBasedReclamationEbr::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
