//! Question #48: Readers-Writers with Writer Preference
//! Category: Concurrency | Difficulty: Hard
//! Concepts: readers-writers, writer preference, starvation, fairness
//! Description: Design an RW lock that prefers writers to avoid writer starvation while preventing reader starvation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ReadersWritersWithWriterPreference {
    data: Mutex<HashMap<i32, i32>>,
}

impl ReadersWritersWithWriterPreference {
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
    fn test_readers_writers_with_writer_preference() {
        let s = ReadersWritersWithWriterPreference::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
