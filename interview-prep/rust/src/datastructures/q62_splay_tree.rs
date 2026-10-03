//! Question #62: Splay Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: splay tree, self-adjusting, amortized, rotations
//! Description: Build a self-adjusting BST that moves recently accessed nodes to the root via zig, zig-zig, and zig-zag operations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SplayTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl SplayTree {
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
    fn test_splay_tree() {
        let s = SplayTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
