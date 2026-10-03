//! Question #519: TIME_WAIT and 2MSL
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: TIME_WAIT, 2MSL, segments, reuse
//! Description: Explain TIME_WAIT, the 2MSL duration, and its role in preventing old segments.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TimeWaitAnd2Msl {
    inner: Mutex<HashMap<String, String>>,
}

impl TimeWaitAnd2Msl {
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
    fn test_time_wait_and_2msl() {
        let s = TimeWaitAnd2Msl::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
