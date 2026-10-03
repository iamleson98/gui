//! Question #11: Ticket Spinlock
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: ticket lock, FIFO fairness, cache-line bounce

use std::sync::atomic::{AtomicU64, Ordering};
use std::hint::spin_loop;

pub struct TicketSpinlock {
    next: AtomicU64,
    now: AtomicU64,
}

impl TicketSpinlock {
    pub const fn new() -> Self {
        Self { next: AtomicU64::new(0), now: AtomicU64::new(0) }
    }

    pub fn lock(&self) {
        let ticket = self.next.fetch_add(1, Ordering::Relaxed);
        while self.now.load(Ordering::Acquire) != ticket {
            spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.now.fetch_add(1, Ordering::Release);
    }

    pub fn try_lock(&self) -> bool {
        let now = self.now.load(Ordering::Acquire);
        let next = self.next.load(Ordering::Acquire);
        if now != next {
            return false;
        }
        self.next.compare_exchange(next, next + 1, Ordering::AcqRel, Ordering::Relaxed).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic() {
        let lock = TicketSpinlock::new();
        lock.lock();
        assert!(!lock.try_lock());
        lock.unlock();
        assert!(lock.try_lock());
        lock.unlock();
    }

    #[test]
    fn test_mutual_exclusion() {
        let lock = Arc::new(TicketSpinlock::new());
        let counter = Arc::new(AtomicU64::new(0));
        let mut handles = vec![];
        for _ in 0..8 {
            let lock = lock.clone();
            let counter = counter.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    lock.lock();
                    counter.fetch_add(1, Ordering::Relaxed);
                    lock.unlock();
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.load(Ordering::Relaxed), 8000);
    }
}
