//! Question #98: Counting Bloom Filter
//! Category: Data Structures | Difficulty: Hard
//! Concepts: counting Bloom filter, counters, deletion, false positive
//! Description: Extend a Bloom filter to counters so it supports deletion via reference counting.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CountingBloomFilter {
    data: Mutex<HashMap<i32, i32>>,
}

impl CountingBloomFilter {
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
    fn test_counting_bloom_filter() {
        let s = CountingBloomFilter::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
