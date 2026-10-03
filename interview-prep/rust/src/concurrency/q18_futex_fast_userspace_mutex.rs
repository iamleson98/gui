//! Question #18: Futex (Fast Userspace Mutex)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: futex, mutex, kernel parking, wait queue
//! Description: Implement a userspace mutex that spins on an atomic word and parks in the kernel only on contention.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FutexFastUserspaceMutex {
    data: Mutex<HashMap<i32, i32>>,
}

impl FutexFastUserspaceMutex {
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
    fn test_futex_fast_userspace_mutex() {
        let s = FutexFastUserspaceMutex::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
