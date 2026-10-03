//! Question #250: Bitmap Index
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: bitmap index, low cardinality, OLAP, rowid
//! Description: Apply bitmap indexes to low-cardinality columns and convert rowids in bulk for OLAP workloads.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BitmapIndex {
    inner: Mutex<HashMap<String, String>>,
}

impl BitmapIndex {
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
    fn test_bitmap_index() {
        let s = BitmapIndex::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
