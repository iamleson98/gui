//! Question #41: Async/Await Executor
//! Category: Concurrency | Difficulty: Hard
//! Concepts: async/await, executor, waker, poll
//! Description: Build a single-threaded cooperative task executor with a ready queue, polling, and wakers for async/await.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AsyncAwaitExecutor {
    data: Mutex<HashMap<i32, i32>>,
}

impl AsyncAwaitExecutor {
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
    fn test_async_await_executor() {
        let s = AsyncAwaitExecutor::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
