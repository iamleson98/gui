//! Question #19: Mutex with Spin-then-Block
//! Category: Concurrency | Difficulty: Hard
//! Concepts: mutex, spin-then-block, futex, latency
//! Description: Design a mutex that spins briefly in userspace and only then issues a system call to park the thread.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MutexWithSpinThenBlock {
    data: Mutex<HashMap<i32, i32>>,
}

impl MutexWithSpinThenBlock {
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
    fn test_mutex_with_spin_then_block() {
        let s = MutexWithSpinThenBlock::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
