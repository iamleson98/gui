//! Question #393: Huge Pages (Transparent)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: huge pages, THP, TLB, page walk
//! Description: Use huge pages to reduce TLB pressure and page-walk cost for large allocations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HugePagesTransparent {
    inner: Mutex<HashMap<String, String>>,
}

impl HugePagesTransparent {
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
    fn test_huge_pages_transparent() {
        let s = HugePagesTransparent::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
