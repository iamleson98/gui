//! Question #2: Michael-Scott Lock-Free MPMC Queue
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: CAS, dummy node, ABA, hazard pointers
//!
//! Description: Construct a linked-list queue supporting concurrent
//! enqueue/dequeue using CAS on head/tail with a sentinel/dummy node.

use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct MSQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> MSQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            let tail_again = self.tail.load(Ordering::Acquire);
            if tail == tail_again {
                if next.is_null() {
                    if unsafe { (*tail).next.compare_exchange_weak(ptr::null_mut(), node, Ordering::Release, Ordering::Relaxed).is_ok() } {
                        let _ = self.tail.compare_exchange(tail, node, Ordering::Release, Ordering::Relaxed);
                        return;
                    }
                } else {
                    let _ = self.tail.compare_exchange(tail, next, Ordering::Release, Ordering::Relaxed);
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };
            let head_again = self.head.load(Ordering::Acquire);
            if head == head_again {
                if head == tail {
                    if next.is_null() {
                        return None;
                    }
                    let _ = self.tail.compare_exchange(tail, next, Ordering::Release, Ordering::Relaxed);
                } else if !next.is_null() {
                    let value = unsafe { (*next).value.take() };
                    if self.head.compare_exchange(head, next, Ordering::Release, Ordering::Relaxed).is_ok() {
                        unsafe { let _ = Box::from_raw(head); }
                        return value;
                    }
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        head == tail && unsafe { (*head).next.load(Ordering::Acquire).is_null() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic() {
        let q = MSQueue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn test_concurrent() {
        let q = Arc::new(MSQueue::new());
        let mut handles = vec![];
        for i in 0..4 {
            let q = q.clone();
            handles.push(thread::spawn(move || {
                for j in 0..1000 {
                    q.enqueue(i * 1000 + j);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let mut count = 0;
        while q.dequeue().is_some() {
            count += 1;
        }
        assert_eq!(count, 4000);
    }
}
