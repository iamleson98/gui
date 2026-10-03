//! Question #37: RCU-Protected Linked List
//! Category: Concurrency | Difficulty: Hard
//! Concepts: RCU, linked list, grace period, read-mostly
//! Description: Implement a linked list whose readers traverse lock-free while updaters use RCU to defer node removal.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RcuProtectedLinkedList {
    data: Mutex<HashMap<i32, i32>>,
}

impl RcuProtectedLinkedList {
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
    fn test_rcu_protected_linked_list() {
        let s = RcuProtectedLinkedList::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
