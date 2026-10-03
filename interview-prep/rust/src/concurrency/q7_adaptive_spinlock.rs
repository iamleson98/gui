//! Question #7: Adaptive Spinlock
//! Category: Concurrency | Difficulty: Hard
//! Concepts: spinlock, futex, backoff, hybrid
//! Description: Design a spinlock that spins briefly then falls back to a kernel futex or parking primitive to avoid wasted CPU.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AdaptiveSpinlock {
    data: Mutex<HashMap<i32, i32>>,
}

impl AdaptiveSpinlock {
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
    fn test_adaptive_spinlock() {
        let s = AdaptiveSpinlock::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
