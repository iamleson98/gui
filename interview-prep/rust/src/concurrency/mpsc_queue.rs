//! Question #1: Lock-Free MPSC Queue
//! Category: Concurrency | Difficulty: Hard | Concepts: CAS, MPSC, ABA
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct MpscQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> MpscQueue<T> {
    pub fn new() -> Self {
        let stub = Box::into_raw(Box::new(Node { value: None, next: AtomicPtr::new(ptr::null_mut()) }));
        Self { head: AtomicPtr::new(stub), tail: AtomicPtr::new(stub) }
    }

    pub fn enqueue(&self, value: T) {
        let node = Box::into_raw(Box::new(Node { value: Some(value), next: AtomicPtr::new(ptr::null_mut()) }));
        let old = self.head.swap(node, Ordering::AcqRel);
        unsafe { (*old).next.store(node, Ordering::Release); }
    }

    pub fn dequeue(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Acquire);
        let next = unsafe { (*tail).next.load(Ordering::Acquire) };
        if next.is_null() { return None; }
        let value = unsafe { (*next).value.take() };
        self.tail.store(next, Ordering::Release);
        value
    }

    pub fn is_empty(&self) -> bool {
        let tail = self.tail.load(Ordering::Acquire);
        unsafe { (*tail).next.load(Ordering::Acquire).is_null() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic() {
        let q = MpscQueue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);
        assert_eq!(q.dequeue(), Some(10));
        assert_eq!(q.dequeue(), Some(20));
        assert_eq!(q.dequeue(), Some(30));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn test_multi_producer() {
        let q = Arc::new(MpscQueue::new());
        let mut handles = vec![];
        for i in 0..8 {
            let q = q.clone();
            handles.push(thread::spawn(move || {
                for j in 0..500 { q.enqueue(i * 500 + j); }
            }));
        }
        for h in handles { h.join().unwrap(); }
        let mut seen = std::collections::HashSet::new();
        let mut count = 0;
        while let Some(v) = q.dequeue() {
            assert!(seen.insert(v), "duplicate {}", v);
            count += 1;
        }
        assert_eq!(count, 4000);
    }
}
