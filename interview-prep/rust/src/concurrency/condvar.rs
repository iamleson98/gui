//! Question #16: Condition Variable
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: condition variable, futex, spurious wakeup, lost wakeup

use std::sync::{Mutex, Condvar};

pub struct CondVar {
    mutex: Mutex<bool>,
    cond: Condvar,
}

impl CondVar {
    pub fn new() -> Self {
        Self {
            mutex: Mutex::new(false),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut flag = self.mutex.lock().unwrap();
        while !*flag {
            flag = self.cond.wait(flag).unwrap();
        }
        *flag = false;
    }

    pub fn signal(&self) {
        let mut flag = self.mutex.lock().unwrap();
        *flag = true;
        self.cond.notify_one();
    }

    pub fn broadcast(&self) {
        let mut flag = self.mutex.lock().unwrap();
        *flag = true;
        self.cond.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let cv = CondVar::new();
        cv.signal();
        cv.wait();
    }
}
