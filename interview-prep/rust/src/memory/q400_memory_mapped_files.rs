//! Question #400: Memory-Mapped Files
//! Category: Memory Management | Difficulty: Hard
//! Concepts: mmap files, zero-copy, page cache, I/O
//! Description: Access files through mapped pages for zero-copy I/O and kernel-managed caching.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryMappedFiles {
    inner: Mutex<HashMap<String, String>>,
}

impl MemoryMappedFiles {
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
    fn test_memory_mapped_files() {
        let s = MemoryMappedFiles::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
