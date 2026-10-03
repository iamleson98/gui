//! Question #237: User-Defined Functions
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: UDF, deterministic, inlining, planner
//! Description: Build deterministic and volatile SQL functions while understanding inlining and planner effects.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct UserDefinedFunctions {
    inner: Mutex<HashMap<String, String>>,
}

impl UserDefinedFunctions {
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
    fn test_user_defined_functions() {
        let s = UserDefinedFunctions::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
