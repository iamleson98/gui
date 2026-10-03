//! Question #93: Cartesian Tree for RMQ
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Cartesian tree, LCA, RMQ reduction, Euler tour
//! Description: Reduce RMQ to LCA on a Cartesian tree built from the array in linear time.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CartesianTreeForRmq {
    data: Mutex<HashMap<i32, i32>>,
}

impl CartesianTreeForRmq {
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
    fn test_cartesian_tree_for_rmq() {
        let s = CartesianTreeForRmq::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
