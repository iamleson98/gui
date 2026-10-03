//! Question #263: Checkpointing Strategies
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: checkpoint, fuzzy, recovery time, LSN
//! Description: Design fuzzy checkpointing to bound recovery time while minimizing foreground pauses.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CheckpointingStrategies {
    inner: Mutex<HashMap<String, String>>,
}

impl CheckpointingStrategies {
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
    fn test_checkpointing_strategies() {
        let s = CheckpointingStrategies::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
