//! Question #101: LRU Cache
//! Category: Data Structures | Difficulty: Hard | Concepts: hash map + doubly-linked list, O(1) eviction
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    key: i32,
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

pub struct LruCache<T> {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<Node<T>>>>,
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Clone> LruCache<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            cache: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> Option<T> {
        if let Some(node) = self.cache.get(&key) {
            let value = node.borrow().value.clone();
            self.move_to_front(node.clone());
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: i32, value: T) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow_mut().value = value;
            let node = node.clone();
            self.move_to_front(node);
            return;
        }
        let node = Rc::new(RefCell::new(Node { key, value, prev: None, next: None }));
        self.cache.insert(key, node.clone());
        self.push_front(node);
        if self.cache.len() > self.capacity {
            self.pop_tail();
        }
    }

    fn move_to_front(&mut self, node: Rc<RefCell<Node<T>>>) {
        // Check if it's already at front
        let is_head = self.head.as_ref().map_or(false, |h| Rc::ptr_eq(h, &node));
        if is_head { return; }
        
        // Detach from current position
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();
        
        if let Some(p) = &prev {
            p.borrow_mut().next = next.clone();
        }
        if let Some(n) = &next {
            n.borrow_mut().prev = prev.clone();
        }
        
        let is_tail = self.tail.as_ref().map_or(false, |t| Rc::ptr_eq(t, &node));
        if is_tail {
            self.tail = prev;
        }
        
        // Push to front
        node.borrow_mut().prev = None;
        self.push_front(node);
    }

    fn push_front(&mut self, node: Rc<RefCell<Node<T>>>) {
        let old_head = self.head.take();
        node.borrow_mut().next = old_head.clone();
        if let Some(old) = old_head {
            old.borrow_mut().prev = Some(node.clone());
        }
        self.head = Some(node);
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }

    fn pop_tail(&mut self) {
        let tail = match self.tail.take() {
            Some(t) => t,
            None => return,
        };
        let prev = tail.borrow().prev.clone();
        match &prev {
            Some(p) => { p.borrow_mut().next = None; }
            None => { self.head = None; }
        }
        self.tail = prev;
        self.cache.remove(&tail.borrow().key);
    }

    pub fn len(&self) -> usize { self.cache.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut c: LruCache<String> = LruCache::new(2);
        c.put(1, "a".to_string());
        c.put(2, "b".to_string());
        assert_eq!(c.get(1), Some("a".to_string()));
        c.put(3, "c".to_string());
        assert_eq!(c.get(2), None);
        assert_eq!(c.get(1), Some("a".to_string()));
        assert_eq!(c.get(3), Some("c".to_string()));
    }
}
