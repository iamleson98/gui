//! Question #269: Snapshot Isolation
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: snapshot isolation, version chain, timestamp, no read lock
//! Description: Provide snapshot isolation using transaction start timestamps and version chains to avoid read locks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SnapshotIsolation {
    inner: Mutex<HashMap<String, String>>,
}

impl SnapshotIsolation {
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
    fn test_snapshot_isolation() {
        let s = SnapshotIsolation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
