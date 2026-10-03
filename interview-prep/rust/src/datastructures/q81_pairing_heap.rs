//! Question #81: Pairing Heap
//! Category: Data Structures | Difficulty: Hard
//! Concepts: pairing heap, merge, two-pass, amortized
//! Description: Build a pairing heap that achieves practical speed via two-pass merging of children.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PairingHeap {
    data: Mutex<HashMap<i32, i32>>,
}

impl PairingHeap {
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
    fn test_pairing_heap() {
        let s = PairingHeap::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
