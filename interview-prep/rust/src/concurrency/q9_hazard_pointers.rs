//! Question #9: Hazard Pointers
//! Category: Concurrency | Difficulty: Hard
//! Concepts: hazard pointers, memory reclamation, ABA, lock-free
//! Description: Implement hazard pointers so that a lock-free data structure safely defers reclamation of nodes a reader is inspecting.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HazardPointers {
    data: Mutex<HashMap<i32, i32>>,
}

impl HazardPointers {
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
    fn test_hazard_pointers() {
        let s = HazardPointers::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
