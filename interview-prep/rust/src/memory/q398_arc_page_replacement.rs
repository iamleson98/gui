//! Question #398: ARC Page Replacement
//! Category: Memory Management | Difficulty: Hard
//! Concepts: ARC, adaptive, recency, frequency
//! Description: Implement the adaptive replacement cache policy balancing recency and frequency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ArcPageReplacement {
    inner: Mutex<HashMap<String, String>>,
}

impl ArcPageReplacement {
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
    fn test_arc_page_replacement() {
        let s = ArcPageReplacement::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
