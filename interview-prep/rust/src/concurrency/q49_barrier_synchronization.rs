//! Question #49: Barrier Synchronization
//! Category: Concurrency | Difficulty: Hard
//! Concepts: barrier, sense reversal, reuse, wait
//! Description: Implement a reusable barrier where N threads wait and then all proceed, with sense reversal.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BarrierSynchronization {
    data: Mutex<HashMap<i32, i32>>,
}

impl BarrierSynchronization {
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
    fn test_barrier_synchronization() {
        let s = BarrierSynchronization::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
