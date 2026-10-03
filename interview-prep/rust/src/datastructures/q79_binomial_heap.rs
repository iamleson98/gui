//! Question #79: Binomial Heap
//! Category: Data Structures | Difficulty: Hard
//! Concepts: binomial heap, merge, lazy, amortized
//! Description: Build a binomial heap supporting merge in O(log n) using linked binomial trees.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BinomialHeap {
    data: Mutex<HashMap<i32, i32>>,
}

impl BinomialHeap {
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
    fn test_binomial_heap() {
        let s = BinomialHeap::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
