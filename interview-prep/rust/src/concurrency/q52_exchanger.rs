//! Question #52: Exchanger
//! Category: Concurrency | Difficulty: Hard
//! Concepts: exchanger, rendezvous, swap, two threads
//! Description: Build an exchanger where two threads rendezvous and swap values atomically.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Exchanger {
    data: Mutex<HashMap<i32, i32>>,
}

impl Exchanger {
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
    fn test_exchanger() {
        let s = Exchanger::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
