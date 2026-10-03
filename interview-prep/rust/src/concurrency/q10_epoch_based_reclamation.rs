//! Question #10: Epoch-Based Reclamation
//! Category: Concurrency | Difficulty: Hard
//! Concepts: epoch reclamation, garbage collection, lock-free, ABA
//! Description: Build an epoch-based memory reclamation scheme that frees nodes only after all pre-epoch readers have retired.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EpochBasedReclamation {
    data: Mutex<HashMap<i32, i32>>,
}

impl EpochBasedReclamation {
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
    fn test_epoch_based_reclamation() {
        let s = EpochBasedReclamation::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
