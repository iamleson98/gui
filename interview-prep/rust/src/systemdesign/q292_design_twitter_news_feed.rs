//! Question #292: Design Twitter / News Feed
//! Category: System Design | Difficulty: Hard
//! Concepts: news feed, fan-out, timeline, ranking
//! Description: Design a timeline service deciding between fan-out on write and fan-out on read.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignTwitterNewsFeed {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignTwitterNewsFeed {
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
    fn test_design_twitter_news_feed() {
        let s = DesignTwitterNewsFeed::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
