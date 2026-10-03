//! Question #419: Superscalar and Out-of-Order Execution
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: superscalar, OoO, ILP, renaming
//! Description: Explain how superscalar and out-of-order execution expose instruction-level parallelism.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SuperscalarAndOutOfOrderExecution {
    inner: Mutex<HashMap<String, String>>,
}

impl SuperscalarAndOutOfOrderExecution {
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
    fn test_superscalar_and_out_of_order_execution() {
        let s = SuperscalarAndOutOfOrderExecution::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
