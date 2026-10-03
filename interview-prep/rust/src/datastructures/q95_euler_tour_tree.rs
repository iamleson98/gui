//! Question #95: Euler Tour Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Euler tour tree, balanced BST, dynamic connectivity, edge
//! Description: Build an Euler tour tree over a balanced BST to support dynamic connectivity.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EulerTourTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl EulerTourTree {
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
    fn test_euler_tour_tree() {
        let s = EulerTourTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
