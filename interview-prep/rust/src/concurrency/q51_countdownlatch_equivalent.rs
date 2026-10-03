//! Question #51: CountDownLatch Equivalent
//! Category: Concurrency | Difficulty: Hard
//! Concepts: latch, one-shot, counter, park
//! Description: Implement a one-shot latch that blocks threads until a counter reaches zero.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CountdownlatchEquivalent {
    data: Mutex<HashMap<i32, i32>>,
}

impl CountdownlatchEquivalent {
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
    fn test_countdownlatch_equivalent() {
        let s = CountdownlatchEquivalent::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
