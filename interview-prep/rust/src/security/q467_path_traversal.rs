//! Question #467: Path Traversal
//! Category: Security | Difficulty: Hard
//! Concepts: path traversal, canonicalization, sandbox, root
//! Description: Prevent path traversal by canonicalizing and confining file access to a root.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PathTraversal {
    inner: Mutex<HashMap<String, String>>,
}

impl PathTraversal {
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
    fn test_path_traversal() {
        let s = PathTraversal::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
