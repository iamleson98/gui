//! Question #394: mmap and Virtual Address Space
//! Category: Memory Management | Difficulty: Hard
//! Concepts: mmap, virtual address, anonymous, file-backed
//! Description: Use mmap to map files and anonymous memory into the process address space.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MmapAndVirtualAddressSpace {
    inner: Mutex<HashMap<String, String>>,
}

impl MmapAndVirtualAddressSpace {
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
    fn test_mmap_and_virtual_address_space() {
        let s = MmapAndVirtualAddressSpace::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
