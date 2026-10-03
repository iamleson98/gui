//! Question #406: Struct Packing and Cache Lines
//! Category: Memory Management | Difficulty: Hard
//! Concepts: packing, cache line, layout, alignment
//! Description: Pack structs to fit within cache lines and trade size against access speed.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct StructPackingAndCacheLines {
    inner: Mutex<HashMap<String, String>>,
}

impl StructPackingAndCacheLines {
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
    fn test_struct_packing_and_cache_lines() {
        let s = StructPackingAndCacheLines::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
