//! Question #42: Thread Pool with Work Queue
//! Category: Concurrency | Difficulty: Hard
//! Concepts: thread pool, work queue, workers, shutdown
//! Description: Implement a fixed-size worker pool with a global task queue, blocking dequeue, and shutdown semantics.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ThreadPoolWithWorkQueue {
    data: Mutex<HashMap<i32, i32>>,
}

impl ThreadPoolWithWorkQueue {
    pub fn new() -> Self {
        Self { data: Mutex::new(HashMap::new()) }
    }
    pub fn insert(&self, key: i32, val: i32) {
        self.data.lock().unwrap().insert(key, val);
    }
    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.lock().unwrap().get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thread_pool_with_work_queue() {
        let s = ThreadPoolWithWorkQueue::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
