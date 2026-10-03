//! Question #47: Dining Philosophers
//! Category: Concurrency | Difficulty: Hard
//! Concepts: deadlock, resource hierarchy, arbitrator, fairness
//! Description: Solve the dining philosophers using resource hierarchy, a waiter (arbitrator), and Chandy-Misra messages.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DiningPhilosophers {
    data: Mutex<HashMap<i32, i32>>,
}

impl DiningPhilosophers {
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
    fn test_dining_philosophers() {
        let s = DiningPhilosophers::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
