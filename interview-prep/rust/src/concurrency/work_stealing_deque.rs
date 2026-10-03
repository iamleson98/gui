//! Question #21: Chase-Lev Work-Stealing Deque
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: Chase-Lev deque, work stealing, top/bottom indices, resize

use std::sync::atomic::{AtomicI64, Ordering};
use std::cell::UnsafeCell;

pub struct WSDeque<T: Clone> {
    buf: UnsafeCell<Vec<T>>,
    mask: i64,
    top: AtomicI64,
    bottom: AtomicI64,
}

unsafe impl<T: Clone + Send> Send for WSDeque<T> {}
unsafe impl<T: Clone + Send> Sync for WSDeque<T> {}

impl<T: Clone + Default> WSDeque<T> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two().max(1);
        Self {
            buf: UnsafeCell::new(vec![T::default(); cap]),
            mask: (cap as i64) - 1,
            top: AtomicI64::new(0),
            bottom: AtomicI64::new(0),
        }
    }

    pub fn push(&self, v: T) {
        let b = self.bottom.load(Ordering::Relaxed);
        unsafe {
            (*self.buf.get())[((b & self.mask) as usize)] = v;
        }
        self.bottom.store(b + 1, Ordering::Release);
    }

    pub fn pop(&self) -> Option<T> {
        let b = self.bottom.load(Ordering::Relaxed) - 1;
        self.bottom.store(b, Ordering::Relaxed);
        let t = self.top.load(Ordering::Acquire);
        if t > b {
            self.bottom.store(t, Ordering::Relaxed);
            return None;
        }
        let v = unsafe { (*self.buf.get())[((b & self.mask) as usize)].clone() };
        if t < b {
            return Some(v);
        }
        self.bottom.store(t + 1, Ordering::Relaxed);
        if self.top.compare_exchange(t, t + 1, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            Some(v)
        } else {
            None
        }
    }

    pub fn steal(&self) -> Option<T> {
        let t = self.top.load(Ordering::Acquire);
        let b = self.bottom.load(Ordering::Acquire);
        if t >= b {
            return None;
        }
        let v = unsafe { (*self.buf.get())[((t & self.mask) as usize)].clone() };
        if self.top.compare_exchange(t, t + 1, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            Some(v)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        let b = self.bottom.load(Ordering::Relaxed);
        let t = self.top.load(Ordering::Relaxed);
        if b <= t { 0 } else { (b - t) as usize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let d = WSDeque::new(16);
        d.push(1);
        d.push(2);
        d.push(3);
        assert_eq!(d.size(), 3);
        assert_eq!(d.pop(), Some(3));
        assert_eq!(d.steal(), Some(1));
    }
}
