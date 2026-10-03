//! Question #303: Design a Web Crawler
//! Category: System Design | Difficulty: Hard
//! Concepts: crawler, frontier, politeness, dedup
//! Description: Design a distributed web crawler with URL frontier scheduling, politeness, and dedup.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAWebCrawler {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAWebCrawler {
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
    fn test_design_a_web_crawler() {
        let s = DesignAWebCrawler::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
