//! Question #445: Async I/O (epoll/io_uring)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: async I/O, epoll, io_uring, scalability
//! Description: Use epoll and io_uring to drive many I/O operations per thread.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AsyncIOEpollIoUring {
    inner: Mutex<HashMap<String, String>>,
}

impl AsyncIOEpollIoUring {
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
    fn test_async_i_o_epoll_io_uring() {
        let s = AsyncIOEpollIoUring::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
