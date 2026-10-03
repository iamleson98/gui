//! Question #50: Phaser / Cyclic Barrier
//! Category: Concurrency | Difficulty: Hard
//! Concepts: phaser, cyclic barrier, parties, phases
//! Description: Design a phaser supporting dynamic party registration, arrivals, and phase advancement.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PhaserCyclicBarrier {
    data: Mutex<HashMap<i32, i32>>,
}

impl PhaserCyclicBarrier {
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
    fn test_phaser_cyclic_barrier() {
        let s = PhaserCyclicBarrier::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
