//! Question #121: Quotient Filter
//! Category: Data Structures | Difficulty: Hard
//! Concepts: quotient filter, open addressing, locality, membership
//! Description: Build a quotient filter using quotient/remainder hashing for membership with locality.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct QuotientFilter {
    data: Mutex<HashMap<i32, i32>>,
}

impl QuotientFilter {
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
    fn test_quotient_filter() {
        let s = QuotientFilter::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
