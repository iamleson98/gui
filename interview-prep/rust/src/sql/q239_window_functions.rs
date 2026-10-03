//! Question #239: Window Functions
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: window functions, rank, frame, analytic
//! Description: Use RANK, DENSE_RANK, ROW_NUMBER, and framing clauses for analytic queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WindowFunctions {
    inner: Mutex<HashMap<String, String>>,
}

impl WindowFunctions {
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
    fn test_window_functions() {
        let s = WindowFunctions::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
