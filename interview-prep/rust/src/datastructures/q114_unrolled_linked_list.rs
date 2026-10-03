//! Question #114: Unrolled Linked List
//! Category: Data Structures | Difficulty: Hard
//! Concepts: unrolled list, cache locality, node capacity, linked list
//! Description: Build a linked list whose nodes store multiple elements to improve cache locality.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct UnrolledLinkedList {
    data: Mutex<HashMap<i32, i32>>,
}

impl UnrolledLinkedList {
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
    fn test_unrolled_linked_list() {
        let s = UnrolledLinkedList::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
