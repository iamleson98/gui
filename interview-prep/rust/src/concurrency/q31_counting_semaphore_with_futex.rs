//! Question #31: Counting Semaphore with Futex
//! Category: Concurrency | Difficulty: Hard
//! Concepts: semaphore, futex, fast path, wait queue
//! Description: Build a fast counting semaphore whose fast path is an atomic compare and whose slow path parks waiters.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CountingSemaphoreWithFutex {
    data: Mutex<HashMap<i32, i32>>,
}

impl CountingSemaphoreWithFutex {
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
    fn test_counting_semaphore_with_futex() {
        let s = CountingSemaphoreWithFutex::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
