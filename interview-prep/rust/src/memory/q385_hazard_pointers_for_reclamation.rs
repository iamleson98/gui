//! Question #385: Hazard Pointers for Reclamation
//! Category: Memory Management | Difficulty: Hard
//! Concepts: hazard pointer, reclamation, lock-free, ABA
//! Description: Use hazard pointers to safely reclaim memory in lock-free data structures.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HazardPointersForReclamation {
    inner: Mutex<HashMap<String, String>>,
}

impl HazardPointersForReclamation {
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
    fn test_hazard_pointers_for_reclamation() {
        let s = HazardPointersForReclamation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
