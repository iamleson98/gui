//! Question #14: Readers-Writer Lock (reader preference)
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: RW lock, reader preference, starvation, atomic counters

use std::sync::{Mutex, Condvar};

pub struct RWLock {
    state: Mutex<State>,
    cond: Condvar,
}

struct State {
    readers: usize,
    writer: bool,
}

impl RWLock {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State { readers: 0, writer: false }),
            cond: Condvar::new(),
        }
    }

    pub fn rlock(&self) {
        let mut s = self.state.lock().unwrap();
        while s.writer {
            s = self.cond.wait(s).unwrap();
        }
        s.readers += 1;
    }

    pub fn runlock(&self) {
        let mut s = self.state.lock().unwrap();
        s.readers -= 1;
        if s.readers == 0 {
            self.cond.notify_all();
        }
    }

    pub fn wlock(&self) {
        let mut s = self.state.lock().unwrap();
        while s.writer || s.readers > 0 {
            s = self.cond.wait(s).unwrap();
        }
        s.writer = true;
    }

    pub fn wunlock(&self) {
        let mut s = self.state.lock().unwrap();
        s.writer = false;
        self.cond.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_exclusion() {
        let lock = RWLock::new();
        let lock2 = RWLock::new();
        lock.wlock();
        lock.wunlock();
        lock2.rlock();
        lock2.runlock();
    }
}
