//! Question #281: Change Data Capture (CDC)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: CDC, WAL, streaming, transaction log
//! Description: Stream row changes from a database by reading the WAL or transaction log.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ChangeDataCaptureCdc {
    inner: Mutex<HashMap<String, String>>,
}

impl ChangeDataCaptureCdc {
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
    fn test_change_data_capture_cdc() {
        let s = ChangeDataCaptureCdc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
