//! Question #302: Design a Search Engine
//! Category: System Design | Difficulty: Hard
//! Concepts: search, crawling, inverted index, ranking
//! Description: Design a search engine with crawling, indexing, ranking, and query serving.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignASearchEngine {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignASearchEngine {
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
    fn test_design_a_search_engine() {
        let s = DesignASearchEngine::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
