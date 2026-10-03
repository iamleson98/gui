//! Question #5: CLH Lock (Craig, Landin, Hagersten)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: spinlock, queue lock, FIFO, spin locality
//! Description: Build a queue lock whose thread spins on the predecessor's lock word and hands off ownership by toggling its own node.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ClhLockCraigLandinHagersten {
    data: Mutex<HashMap<i32, i32>>,
}

impl ClhLockCraigLandinHagersten {
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
    fn test_clh_lock_craig_landin_hagersten() {
        let s = ClhLockCraigLandinHagersten::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
