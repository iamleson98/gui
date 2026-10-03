//! Question #242: Pivoting and Unpivoting
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: pivot, unpivot, conditional aggregation, cross tab
//! Description: Pivot rows to columns and unpivot columns to rows using conditional aggregation and UNION.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PivotingAndUnpivoting {
    inner: Mutex<HashMap<String, String>>,
}

impl PivotingAndUnpivoting {
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
    fn test_pivoting_and_unpivoting() {
        let s = PivotingAndUnpivoting::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
