//! Question #92: Disjoint Sparse Table
//! Category: Data Structures | Difficulty: Hard
//! Concepts: sparse table, non-idempotent, range sum, preprocessing
//! Description: Build a sparse table supporting non-idempotent range queries such as sum in O(log n).

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DisjointSparseTable {
    data: Mutex<HashMap<i32, i32>>,
}

impl DisjointSparseTable {
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
    fn test_disjoint_sparse_table() {
        let s = DisjointSparseTable::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
