//! Question #59: NUMA-Aware Synchronization
//! Category: Concurrency | Difficulty: Hard
//! Concepts: NUMA, topology, data placement, scalability
//! Description: Design locks and data placement that respect NUMA topology to reduce cross-socket traffic.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct NumaAwareSynchronization {
    data: Mutex<HashMap<i32, i32>>,
}

impl NumaAwareSynchronization {
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
    fn test_numa_aware_synchronization() {
        let s = NumaAwareSynchronization::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
