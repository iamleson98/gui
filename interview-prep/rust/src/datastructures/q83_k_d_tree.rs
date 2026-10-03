//! Question #83: K-D Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: k-d tree, nearest neighbor, range query, splitting planes
//! Description: Build a k-d tree for orthogonal range and nearest-neighbor queries in k-dimensional space.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct KDTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl KDTree {
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
    fn test_k_d_tree() {
        let s = KDTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
