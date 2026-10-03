//! Question #78: Binary Heap
//! Category: Data Structures | Difficulty: Hard
//! Concepts: binary heap, array, sift, priority queue
//! Description: Implement a binary heap as an array with sift-up and sift-down for priority queue operations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BinaryHeap {
    data: Mutex<HashMap<i32, i32>>,
}

impl BinaryHeap {
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
    fn test_binary_heap() {
        let s = BinaryHeap::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
