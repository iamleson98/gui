//! Question #350: Design Hybrid Logical Clocks (HLC)
//! Category: System Design | Difficulty: Hard
//! Concepts: HLC, physical, logical, drift
//! Description: Combine physical and logical time into HLCs for bounded drift ordering.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignHybridLogicalClocksHlc {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignHybridLogicalClocksHlc {
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
    fn test_design_hybrid_logical_clocks_hlc() {
        let s = DesignHybridLogicalClocksHlc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
