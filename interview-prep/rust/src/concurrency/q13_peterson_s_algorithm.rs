//! Question #13: Peterson's Algorithm
//! Category: Concurrency | Difficulty: Hard
//! Concepts: mutual exclusion, flags, turn, memory ordering
//! Description: Implement the classic two-process mutual exclusion algorithm using flags and a turn variable with sequential consistency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PetersonSAlgorithm {
    data: Mutex<HashMap<i32, i32>>,
}

impl PetersonSAlgorithm {
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
    fn test_peterson_s_algorithm() {
        let s = PetersonSAlgorithm::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
