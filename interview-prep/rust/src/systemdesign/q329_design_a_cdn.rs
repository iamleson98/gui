//! Question #329: Design a CDN
//! Category: System Design | Difficulty: Hard
//! Concepts: CDN, edge cache, origin pull, invalidation
//! Description: Design a CDN with edge caches, origin pull, and cache invalidation strategies.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignACdn {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignACdn {
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
    fn test_design_a_cdn() {
        let s = DesignACdn::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
