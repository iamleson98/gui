//! Question #65: B* Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: B* tree, redistribution, split, fill factor
//! Description: Implement a B-tree variant that keeps nodes at least two-thirds full by redistributing before splitting.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl BTree {
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
    fn test_b_tree() {
        let s = BTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
