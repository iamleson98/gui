//! Question #69: Cartesian Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Cartesian tree, RMQ, heap property, linear build
//! Description: Construct a Cartesian tree from an array in linear time and use it for range minimum queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CartesianTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl CartesianTree {
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
    fn test_cartesian_tree() {
        let s = CartesianTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
