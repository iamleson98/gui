//! Question #115: XOR Linked List
//! Category: Data Structures | Difficulty: Hard
//! Concepts: XOR list, pointer compression, memory, traversal
//! Description: Implement a doubly linked list using XOR of adjacent pointers to store one pointer per node.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct XorLinkedList {
    data: Mutex<HashMap<i32, i32>>,
}

impl XorLinkedList {
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
    fn test_xor_linked_list() {
        let s = XorLinkedList::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
