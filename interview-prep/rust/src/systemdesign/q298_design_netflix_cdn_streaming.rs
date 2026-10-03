//! Question #298: Design Netflix / CDN Streaming
//! Category: System Design | Difficulty: Hard
//! Concepts: streaming CDN, origin shield, cache tier, ABR
//! Description: Design a streaming CDN with origin shields, caching tiers, and ABR playback.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignNetflixCdnStreaming {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignNetflixCdnStreaming {
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
    fn test_design_netflix_cdn_streaming() {
        let s = DesignNetflixCdnStreaming::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
