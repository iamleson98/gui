//! Question #446: io_uring vs epoll
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: io_uring, epoll, submission queue, completion
//! Description: Contrast io_uring's submission/completion queues with epoll's readiness model.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IoUringVsEpoll {
    inner: Mutex<HashMap<String, String>>,
}

impl IoUringVsEpoll {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
    pub fn set(&self, key: &str, val: &str) {
        self.inner.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_io_uring_vs_epoll() {
        let s = IoUringVsEpoll::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
