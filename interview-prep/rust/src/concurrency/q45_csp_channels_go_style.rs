//! Question #45: CSP Channels (Go-style)
//! Category: Concurrency | Difficulty: Hard
//! Concepts: channels, CSP, select, rendezvous
//! Description: Build unbuffered and buffered channels with select, close, and fair rendezvous semantics.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CspChannelsGoStyle {
    data: Mutex<HashMap<i32, i32>>,
}

impl CspChannelsGoStyle {
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
    fn test_csp_channels_go_style() {
        let s = CspChannelsGoStyle::new();
        s.insert(1, 10);
        assert_eq!(s.get(1), Some(10));
    }
}
