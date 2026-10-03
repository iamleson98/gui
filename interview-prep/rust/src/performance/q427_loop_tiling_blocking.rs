//! Question #427: Loop Tiling / Blocking
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: tiling, blocking, cache, matrix
//! Description: Tile nested loops to fit working sets in cache for matrix computations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LoopTilingBlocking {
    inner: Mutex<HashMap<String, String>>,
}

impl LoopTilingBlocking {
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
    fn test_loop_tiling_blocking() {
        let s = LoopTilingBlocking::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
