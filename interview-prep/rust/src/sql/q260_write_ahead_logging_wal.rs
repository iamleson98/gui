//! Question #260: Write-Ahead Logging (WAL)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: WAL, redo log, durability, flush order
//! Description: Implement WAL semantics so that no data page is flushed before its redo log record.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WriteAheadLoggingWal {
    inner: Mutex<HashMap<String, String>>,
}

impl WriteAheadLoggingWal {
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
    fn test_write_ahead_logging_wal() {
        let s = WriteAheadLoggingWal::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
