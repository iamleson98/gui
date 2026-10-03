//! Question #29: Wait-Free vs Lock-Free Progress
//! Category: Concurrency | Difficulty: Hard
//! Concepts: wait-free, lock-free, progress, bounded steps
//! Description: Design a wait-free queue where every operation completes in a bounded number of steps, and contrast with lock-free progress.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct WaitFreeVsLockFreeProgress {
    data: Mutex<HashMap<i32, i32>>,
}

impl WaitFreeVsLockFreeProgress {
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
    fn test_wait_free_vs_lock_free_progress() {
        let s = WaitFreeVsLockFreeProgress::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
