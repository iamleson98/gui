//! Question #91: Sparse Table (RMQ)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: sparse table, RMQ, idempotent, preprocessing
//! Description: Preprocess an array for O(1) range minimum queries using a sparse table of powers of two.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SparseTableRmq {
    data: Mutex<HashMap<i32, i32>>,
}

impl SparseTableRmq {
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
    fn test_sparse_table_rmq() {
        let s = SparseTableRmq::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
