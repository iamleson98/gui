//! Question #294: Design Instagram / Photo Sharing
//! Category: System Design | Difficulty: Hard
//! Concepts: photo sharing, object storage, CDN, metadata
//! Description: Design a photo-sharing service with object storage, CDN, and metadata sharding.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignInstagramPhotoSharing {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignInstagramPhotoSharing {
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
    fn test_design_instagram_photo_sharing() {
        let s = DesignInstagramPhotoSharing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
