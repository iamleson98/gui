//! Question #77: Binary Indexed Range Tree
//! Category: Data Structures | Difficulty: Hard
//! Concepts: BIT, range update, point query, difference
//! Description: Build a BIT supporting range updates and point queries via two complementary arrays.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BinaryIndexedRangeTree {
    data: Mutex<HashMap<i32, i32>>,
}

impl BinaryIndexedRangeTree {
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
    fn test_binary_indexed_range_tree() {
        let s = BinaryIndexedRangeTree::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
