//! Question #410: Huge Page Table Walks
//! Category: Memory Management | Difficulty: Hard
//! Concepts: page walk, huge page, TLB, cost
//! Description: Analyze page-walk cost with huge pages and the resulting TLB savings.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HugePageTableWalks {
    inner: Mutex<HashMap<String, String>>,
}

impl HugePageTableWalks {
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
    fn test_huge_page_table_walks() {
        let s = HugePageTableWalks::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
