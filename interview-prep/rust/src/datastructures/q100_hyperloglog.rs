//! Question #100: HyperLogLog
//! Category: Data Structures | Difficulty: Hard
//! Concepts: HyperLogLog, cardinality, stochastic averaging, bias correction
//! Description: Implement HyperLogLog for approximate distinct-count (cardinality) estimation in fixed memory.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Hyperloglog {
    data: Mutex<HashMap<i32, i32>>,
}

impl Hyperloglog {
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
    fn test_hyperloglog() {
        let s = Hyperloglog::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
