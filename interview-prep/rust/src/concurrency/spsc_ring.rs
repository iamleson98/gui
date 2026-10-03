//! SPSC Ring Buffer — single-producer, single-consumer bounded queue.
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct SpscRing<T> {
    buf: std::cell::UnsafeCell<Vec<T>>,
    mask: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T: Send> Send for SpscRing<T> {}
unsafe impl<T: Send> Sync for SpscRing<T> {}

impl<T: Clone + Default> SpscRing<T> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two();
        Self {
            buf: std::cell::UnsafeCell::new(vec![T::default(); cap]),
            mask: cap - 1,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn try_enqueue(&self, v: T) -> bool {
        let h = self.head.load(Ordering::Relaxed);
        let t = self.tail.load(Ordering::Acquire);
        if h.wrapping_sub(t) > self.mask {
            return false;
        }
        unsafe {
            (*self.buf.get())[h & self.mask] = v;
        }
        self.head.store(h.wrapping_add(1), Ordering::Release);
        true
    }

    pub fn try_dequeue(&self) -> Option<T> {
        let t = self.tail.load(Ordering::Relaxed);
        let h = self.head.load(Ordering::Acquire);
        if h == t {
            return None;
        }
        let v = unsafe { (*self.buf.get())[t & self.mask].clone() };
        self.tail.store(t.wrapping_add(1), Ordering::Release);
        Some(v)
    }

    pub fn len(&self) -> usize {
        self.head.load(Ordering::Relaxed).wrapping_sub(self.tail.load(Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let r = SpscRing::new(8);
        for i in 0..8 {
            assert!(r.try_enqueue(i));
        }
        assert!(!r.try_enqueue(99));
        for i in 0..8 {
            assert_eq!(r.try_dequeue(), Some(i));
        }
        assert_eq!(r.try_dequeue(), None);
    }

    #[test]
    fn test_interleaved() {
        let r = SpscRing::new(4);
        r.try_enqueue(1);
        r.try_enqueue(2);
        assert_eq!(r.try_dequeue(), Some(1));
        r.try_enqueue(3);
        r.try_enqueue(4);
        for &exp in &[2, 3, 4] {
            assert_eq!(r.try_dequeue(), Some(exp));
        }
    }
}
