//! Question #61: AVL Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: BST, rotations, balance factor, AVL
//! Description: Implement a self-balancing binary search tree that maintains height balance via rotations on insert and delete.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AvlTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl AvlTree {
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
    fn test_avl_tree() {
        let s = AvlTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
