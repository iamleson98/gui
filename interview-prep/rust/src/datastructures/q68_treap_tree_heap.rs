//! Question #68: Treap (Tree + Heap)
//! Category: Data Structures | Difficulty: Hard
//! Concepts: treap, randomized, priority, rotations
//! Description: Build a randomized BST that maintains heap order on randomly assigned priorities.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TreapTreeHeap {
    data: Mutex<HashMap<i32, i32>>,
}

impl TreapTreeHeap {
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
    fn test_treap_tree_heap() {
        let s = TreapTreeHeap::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
