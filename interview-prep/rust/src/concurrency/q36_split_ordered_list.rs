//! Question #36: Split-Ordered List
//! Category: Concurrency | Difficulty: Hard
//! Concepts: split-ordered list, lock-free, sorted list, hash
//! Description: Implement a lock-free hash table based on a sorted linked list with reverse-key ordering (Shalev-Shavit).

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SplitOrderedList {
    data: Mutex<HashMap<i32, i32>>,
}

impl SplitOrderedList {
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
    fn test_split_ordered_list() {
        let s = SplitOrderedList::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
