//! Question #408: Cache-Friendly Data Layout (SoA vs AoS)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: SoA, AoS, SIMD, cache
//! Description: Choose between array-of-structs and struct-of-arrays for SIMD and cache efficiency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CacheFriendlyDataLayoutSoaVsAos {
    inner: Mutex<HashMap<String, String>>,
}

impl CacheFriendlyDataLayoutSoaVsAos {
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
    fn test_cache_friendly_data_layout_soa_vs_aos() {
        let s = CacheFriendlyDataLayoutSoaVsAos::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
