//! Lock-Free Treiber Stack — CAS-based LIFO stack.
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

type Node<T> = Box<NodeData<T>>;
struct NodeData<T> {
    value: T,
    next: *mut NodeData<T>,
}

pub struct TreiberStack<T> {
    head: AtomicPtr<NodeData<T>>,
}

impl<T> TreiberStack<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, value: T) {
        let node = Box::into_raw(Box::new(NodeData { value, next: ptr::null_mut() }));
        loop {
            let old = self.head.load(Ordering::Acquire);
            unsafe { (*node).next = old; }
            if self.head.compare_exchange_weak(old, node, Ordering::Release, Ordering::Relaxed).is_ok() {
                return;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let old = self.head.load(Ordering::Acquire);
            if old.is_null() {
                return None;
            }
            let next = unsafe { (*old).next };
            if self.head.compare_exchange_weak(old, next, Ordering::Release, Ordering::Relaxed).is_ok() {
                let node = unsafe { Box::from_raw(old) };
                return Some(node.value);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Default for TreiberStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic() {
        let s = TreiberStack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn test_concurrent() {
        let s = Arc::new(TreiberStack::new());
        let mut handles = vec![];
        for i in 0..8 {
            let s = s.clone();
            handles.push(thread::spawn(move || {
                for j in 0..1000 {
                    s.push(i * 1000 + j);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let mut count = 0;
        while s.pop().is_some() {
            count += 1;
        }
        assert_eq!(count, 8000);
    }
}
