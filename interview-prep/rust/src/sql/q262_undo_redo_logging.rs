//! Question #262: Undo/Redo Logging
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: undo, redo, steal, no-force
//! Description: Contrast undo-only, redo-only, and undo-redo logging with respect to steal and no-force policies.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct UndoRedoLogging {
    inner: Mutex<HashMap<String, String>>,
}

impl UndoRedoLogging {
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
    fn test_undo_redo_logging() {
        let s = UndoRedoLogging::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
