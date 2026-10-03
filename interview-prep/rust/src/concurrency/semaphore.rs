//! Counting Semaphore using std::sync.
use std::sync::{Arc, Condvar, Mutex};

pub struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
}

impl Semaphore {
    pub fn new(n: usize) -> Self {
        Self {
            count: Mutex::new(n),
            cond: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cond.notify_one();
    }

    pub fn try_acquire(&self) -> bool {
        let mut count = self.count.lock().unwrap();
        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic() {
        let s = Semaphore::new(3);
        s.acquire();
        s.acquire();
        s.acquire();
        assert!(!s.try_acquire());
        s.release();
        assert!(s.try_acquire());
    }

    #[test]
    fn test_concurrency() {
        let s = Arc::new(Semaphore::new(4));
        let mut active = Arc::new(Mutex::new(0));
        let mut max_active = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..20 {
            let s = s.clone();
            let active = active.clone();
            let max_active = max_active.clone();
            handles.push(thread::spawn(move || {
                s.acquire();
                {
                    let mut a = active.lock().unwrap();
                    *a += 1;
                    let mut m = max_active.lock().unwrap();
                    if *a > *m {
                        *m = *a;
                    }
                }
                thread::sleep(std::time::Duration::from_millis(1));
                {
                    let mut a = active.lock().unwrap();
                    *a -= 1;
                }
                s.release();
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert!(*max_active.lock().unwrap() <= 4);
    }
}
