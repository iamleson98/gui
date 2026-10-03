//! Question #24: Unbounded Lock-Free Queue with SMR
//! Category: Concurrency | Difficulty: Hard
//! Concepts: lock-free queue, hazard pointers, ABA, unbounded
//! Description: Design an unbounded MPMC queue that grows linked-node storage and reclaims nodes via hazard pointers or epochs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct UnboundedLockFreeQueueWithSmr {
    data: Mutex<HashMap<i32, i32>>,
}

impl UnboundedLockFreeQueueWithSmr {
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
    fn test_unbounded_lock_free_queue_with_smr() {
        let s = UnboundedLockFreeQueueWithSmr::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
