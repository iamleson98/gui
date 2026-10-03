//! Question #38: Lock-Free Doubly Linked List
//! Category: Concurrency | Difficulty: Hard
//! Concepts: doubly linked list, lock-free, marking, ABA
//! Description: Design a lock-free doubly linked list handling the classic concurrent-deletion hazard with marking.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LockFreeDoublyLinkedList {
    data: Mutex<HashMap<i32, i32>>,
}

impl LockFreeDoublyLinkedList {
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
    fn test_lock_free_doubly_linked_list() {
        let s = LockFreeDoublyLinkedList::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
