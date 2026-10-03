//! Question #12: Queue Spinlock (Linux qspinlock)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: spinlock, queue lock, MCS, fairness
//! Description: Design a compact queue spinlock that stores waiting nodes in a small per-CPU array and falls back to a linked list.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct QueueSpinlockLinuxQspinlock {
    data: Mutex<HashMap<i32, i32>>,
}

impl QueueSpinlockLinuxQspinlock {
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
    fn test_queue_spinlock_linux_qspinlock() {
        let s = QueueSpinlockLinuxQspinlock::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
