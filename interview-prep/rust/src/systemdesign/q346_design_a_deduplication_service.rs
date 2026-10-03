//! Question #346: Design a Deduplication Service
//! Category: System Design | Difficulty: Hard
//! Concepts: dedup, content hash, windowed, idempotency
//! Description: Design a service that deduplicates events using content hashing and a windowed store.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADeduplicationService {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADeduplicationService {
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
    fn test_design_a_deduplication_service() {
        let s = DesignADeduplicationService::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
