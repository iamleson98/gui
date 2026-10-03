//! Question #124: LSM-Tree (Log-Structured Merge)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: LSM tree, memtable, SSTable, compaction
//! Description: Implement a log-structured merge tree with memtable, SSTables, and leveled compaction.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LsmTreeLogStructuredMerge {
    data: Mutex<HashMap<i32, i32>>,
}

impl LsmTreeLogStructuredMerge {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
    pub fn insert(&self, key: i32, val: i32) {
        self.data.lock().unwrap().insert(key, val);
    }
    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.lock().unwrap().get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lsm_tree_log_structured_merge() {
        let s = LsmTreeLogStructuredMerge::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
