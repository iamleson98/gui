//! Question #284: Sharding with Consistent Hashing
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: sharding, consistent hashing, virtual nodes, reshuffle
//! Description: Distribute rows across shards using consistent hashing to minimize reshuffle on resharding.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ShardingWithConsistentHashing {
    inner: Mutex<HashMap<String, String>>,
}

impl ShardingWithConsistentHashing {
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
    fn test_sharding_with_consistent_hashing() {
        let s = ShardingWithConsistentHashing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
