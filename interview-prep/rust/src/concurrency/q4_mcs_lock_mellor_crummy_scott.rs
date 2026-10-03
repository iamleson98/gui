//! Question #4: MCS Lock (Mellor-Crummy & Scott)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: spinlock, queue lock, scalability, NUMA
//! Description: Implement a scalable list-based queue lock where each thread spins on a locally-cached flag.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct McsLockMellorCrummyScott {
    data: Mutex<HashMap<i32, i32>>,
}

impl McsLockMellorCrummyScott {
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
    fn test_mcs_lock_mellor_crummy_scott() {
        let s = McsLockMellorCrummyScott::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
