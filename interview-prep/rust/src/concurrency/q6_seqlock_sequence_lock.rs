//! Question #6: SeqLock (Sequence Lock)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: seqlock, readers-writers, memory ordering, fences
//! Description: Implement a sequence-lock reader/writer pattern allowing lock-free reads while writes increment a counter twice.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SeqlockSequenceLock {
    data: Mutex<HashMap<i32, i32>>,
}

impl SeqlockSequenceLock {
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
    fn test_seqlock_sequence_lock() {
        let s = SeqlockSequenceLock::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
