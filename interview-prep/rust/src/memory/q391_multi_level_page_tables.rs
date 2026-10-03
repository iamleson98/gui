//! Question #391: Multi-Level Page Tables
//! Category: Memory Management | Difficulty: Hard
//! Concepts: multi-level page table, sparse, compact, translation
//! Description: Design multi-level page tables to compactly represent sparse virtual address spaces.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MultiLevelPageTables {
    inner: Mutex<HashMap<String, String>>,
}

impl MultiLevelPageTables {
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
    fn test_multi_level_page_tables() {
        let s = MultiLevelPageTables::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
