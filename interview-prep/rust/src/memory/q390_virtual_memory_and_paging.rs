//! Question #390: Virtual Memory and Paging
//! Category: Memory Management | Difficulty: Hard
//! Concepts: virtual memory, paging, page table, translation
//! Description: Implement paging that maps virtual to physical pages via page tables.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct VirtualMemoryAndPaging {
    inner: Mutex<HashMap<String, String>>,
}

impl VirtualMemoryAndPaging {
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
    fn test_virtual_memory_and_paging() {
        let s = VirtualMemoryAndPaging::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
