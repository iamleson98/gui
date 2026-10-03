//! Question #295: Design Dropbox / File Storage
//! Category: System Design | Difficulty: Hard
//! Concepts: file sync, chunking, dedup, conflict
//! Description: Design a file-sync service with chunking, deduplication, and conflict resolution.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignDropboxFileStorage {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignDropboxFileStorage {
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
    fn test_design_dropbox_file_storage() {
        let s = DesignDropboxFileStorage::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
