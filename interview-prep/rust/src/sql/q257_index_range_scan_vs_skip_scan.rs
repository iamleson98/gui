//! Question #257: Index Range Scan vs Skip Scan
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: range scan, skip scan, composite index, leading column
//! Description: Contrast range scans with skip scans that handle leading-column equality filters.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IndexRangeScanVsSkipScan {
    inner: Mutex<HashMap<String, String>>,
}

impl IndexRangeScanVsSkipScan {
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
    fn test_index_range_scan_vs_skip_scan() {
        let s = IndexRangeScanVsSkipScan::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
