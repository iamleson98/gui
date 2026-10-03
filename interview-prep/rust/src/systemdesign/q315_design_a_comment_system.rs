//! Question #315: Design a Comment System
//! Category: System Design | Difficulty: Hard
//! Concepts: comments, threading, materialized path, pagination
//! Description: Design a threaded comment system with materialized paths and pagination.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignACommentSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignACommentSystem {
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
    fn test_design_a_comment_system() {
        let s = DesignACommentSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
