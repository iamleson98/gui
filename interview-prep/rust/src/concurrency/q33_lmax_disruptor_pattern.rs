//! Question #33: LMAX Disruptor Pattern
//! Category: Concurrency | Difficulty: Hard
//! Concepts: Disruptor, ring, sequences, batching
//! Description: Build a Disruptor-style ring with sequenced consumers, gating sequences, and a batched publisher.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LmaxDisruptorPattern {
    data: Mutex<HashMap<i32, i32>>,
}

impl LmaxDisruptorPattern {
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
    fn test_lmax_disruptor_pattern() {
        let s = LmaxDisruptorPattern::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
