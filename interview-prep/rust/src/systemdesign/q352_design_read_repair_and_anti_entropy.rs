//! Question #352: Design Read-Repair and Anti-Entropy
//! Category: System Design | Difficulty: Hard
//! Concepts: read repair, anti-entropy, replica, consistency
//! Description: Repair divergent replicas via read-repair and background anti-entropy.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignReadRepairAndAntiEntropy {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignReadRepairAndAntiEntropy {
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
    fn test_design_read_repair_and_anti_entropy() {
        let s = DesignReadRepairAndAntiEntropy::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
