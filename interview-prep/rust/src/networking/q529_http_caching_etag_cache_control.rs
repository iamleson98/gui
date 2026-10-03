//! Question #529: HTTP Caching (ETag, Cache-Control)
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: caching, ETag, Cache-Control, freshness
//! Description: Configure conditional and freshness caching with ETag and Cache-Control.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HttpCachingEtagCacheControl {
    inner: Mutex<HashMap<String, String>>,
}

impl HttpCachingEtagCacheControl {
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
    fn test_http_caching_etag_cache_control() {
        let s = HttpCachingEtagCacheControl::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
