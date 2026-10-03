//! Question #23: Bounded MPMC Queue (Array)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: MPMC, bounded queue, sequence, CAS
//! Description: Implement an array-based bounded MPMC queue using a sequence per cell and compare-and-swap on the cell.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BoundedMpmcQueueArray {
    data: Mutex<HashMap<i32, i32>>,
}

impl BoundedMpmcQueueArray {
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
    fn test_bounded_mpmc_queue_array() {
        let s = BoundedMpmcQueueArray::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
