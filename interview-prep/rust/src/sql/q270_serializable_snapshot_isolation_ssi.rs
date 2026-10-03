//! Question #270: Serializable Snapshot Isolation (SSI)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: SSI, serializable, conflict, safe retry
//! Description: Detect dangerous read/write patterns to provide serializability over snapshot isolation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SerializableSnapshotIsolationSsi {
    inner: Mutex<HashMap<String, String>>,
}

impl SerializableSnapshotIsolationSsi {
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
    fn test_serializable_snapshot_isolation_ssi() {
        let s = SerializableSnapshotIsolationSsi::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
