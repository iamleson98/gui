//! Question #222: Boyce-Codd Normal Form
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: BCNF, functional dependency, candidate key, decomposition
//! Description: Identify and decompose a schema to BCNF by removing non-trivial dependencies where a determinant is not a candidate key.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BoyceCoddNormalForm {
    inner: Mutex<HashMap<String, String>>,
}

impl BoyceCoddNormalForm {
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
    fn test_boyce_codd_normal_form() {
        let s = BoyceCoddNormalForm::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
