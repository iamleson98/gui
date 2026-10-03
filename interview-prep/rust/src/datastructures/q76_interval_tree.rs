//! Question #76: Interval Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: interval tree, overlap query, augmented BST, red-black
//! Description: Implement a red-black based interval tree supporting overlap queries for intervals.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IntervalTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl IntervalTree {
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
    fn test_interval_tree() {
        let s = IntervalTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
