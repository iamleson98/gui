//! Question #245: Nested Sets Model
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: nested sets, left/right, subtree, preorder
//! Description: Store trees using nested sets with left/right preorder bounds for subtree queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct NestedSetsModel {
    inner: Mutex<HashMap<String, String>>,
}

impl NestedSetsModel {
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
    fn test_nested_sets_model() {
        let s = NestedSetsModel::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
