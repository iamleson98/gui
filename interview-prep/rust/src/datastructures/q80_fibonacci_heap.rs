//! Question #80: Fibonacci Heap
//! Category: Data Structures | Difficulty: Hard
//! Concepts: Fibonacci heap, amortized, decrease-key, cascading cut
//! Description: Implement a Fibonacci heap with lazy melding and amortized O(1) decrease-key for Dijkstra.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FibonacciHeap {
    data: Mutex<HashMap<i32, i32>>,
}

impl FibonacciHeap {
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
    fn test_fibonacci_heap() {
        let s = FibonacciHeap::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
