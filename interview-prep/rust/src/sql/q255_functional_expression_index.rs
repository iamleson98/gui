//! Question #255: Functional/Expression Index
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: expression index, function, immutable, predicate
//! Description: Index the result of an expression or function to accelerate transformed predicates.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FunctionalExpressionIndex {
    inner: Mutex<HashMap<String, String>>,
}

impl FunctionalExpressionIndex {
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
    fn test_functional_expression_index() {
        let s = FunctionalExpressionIndex::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
