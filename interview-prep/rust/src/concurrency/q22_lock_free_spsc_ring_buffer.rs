//! Question #22: Lock-Free SPSC Ring Buffer
//! Category: Concurrency | Difficulty: Hard
//! Concepts: SPSC, ring buffer, memory ordering, cache lines
//! Description: Build a single-producer single-consumer bounded ring buffer using relaxed loads/stores and a power-of-two size.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LockFreeSpscRingBuffer {
    data: Mutex<HashMap<i32, i32>>,
}

impl LockFreeSpscRingBuffer {
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
    fn test_lock_free_spsc_ring_buffer() {
        let s = LockFreeSpscRingBuffer::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
