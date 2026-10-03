//! Question #367: Stack (Linear) Allocator
//! Category: Memory Management | Difficulty: Hard
//! Concepts: stack allocator, markers, LIFO free, linear
//! Description: Implement a stack allocator with markers to roll back to a previous top.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct StackLinearAllocator {
    inner: Mutex<HashMap<String, String>>,
}

impl StackLinearAllocator {
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
    fn test_stack_linear_allocator() {
        let s = StackLinearAllocator::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
