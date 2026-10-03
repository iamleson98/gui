//! Question #238: Views: Materialized vs Virtual
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: views, materialized, refresh, abstraction
//! Description: Compare materialized and virtual views for query abstraction and refresh strategies.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ViewsMaterializedVsVirtual {
    inner: Mutex<HashMap<String, String>>,
}

impl ViewsMaterializedVsVirtual {
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
    fn test_views_materialized_vs_virtual() {
        let s = ViewsMaterializedVsVirtual::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
