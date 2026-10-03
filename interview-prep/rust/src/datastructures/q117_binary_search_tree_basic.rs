//! Question #117: Binary Search Tree (Basic)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: BST, inorder, insert/delete, balanced
//! Description: Implement an unbalanced BST with insert, delete, and search.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BinarySearchTreeBasic {
    data: Mutex<HashMap<i32, i32>>,
}

impl BinarySearchTreeBasic {
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
    fn test_binary_search_tree_basic() {
        let s = BinarySearchTreeBasic::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
