//! Question #399: Working Set and RSS
//! Category: Memory Management | Difficulty: Hard
//! Concepts: working set, RSS, thrashing, sizing
//! Description: Estimate the working set size and resident set to size memory and detect thrashing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WorkingSetAndRss {
    inner: Mutex<HashMap<String, String>>,
}

impl WorkingSetAndRss {
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
    fn test_working_set_and_rss() {
        let s = WorkingSetAndRss::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
