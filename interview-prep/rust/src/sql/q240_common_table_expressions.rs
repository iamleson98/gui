//! Question #240: Common Table Expressions
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: CTE, readability, materialization, recursion
//! Description: Refactor complex queries with CTEs for readability and understand materialization behavior.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CommonTableExpressions {
    inner: Mutex<HashMap<String, String>>,
}

impl CommonTableExpressions {
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
    fn test_common_table_expressions() {
        let s = CommonTableExpressions::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
