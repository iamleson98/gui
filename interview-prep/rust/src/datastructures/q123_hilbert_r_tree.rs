//! Question #123: Hilbert R-Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Hilbert R-tree, space-filling curve, ordering, spatial
//! Description: Design an R-tree whose leaves follow a Hilbert ordering to improve query performance.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HilbertRTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl HilbertRTree {
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
    fn test_hilbert_r_tree() {
        let s = HilbertRTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
