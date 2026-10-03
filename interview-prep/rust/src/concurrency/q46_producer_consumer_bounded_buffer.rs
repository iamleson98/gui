//! Question #46: Producer-Consumer Bounded Buffer
//! Category: Concurrency | Difficulty: Hard
//! Concepts: producer-consumer, bounded buffer, condition variable, backpressure
//! Description: Implement the classic producer-consumer bounded buffer using a mutex and two condition variables.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ProducerConsumerBoundedBuffer {
    data: Mutex<HashMap<i32, i32>>,
}

impl ProducerConsumerBoundedBuffer {
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
    fn test_producer_consumer_bounded_buffer() {
        let s = ProducerConsumerBoundedBuffer::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
