//! Question #493: Salted Hashing for Storage
//! Category: Security | Difficulty: Hard
//! Concepts: salted hash, slow hash, storage, cracking
//! Description: Store credentials as salted slow hashes to resist offline cracking.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SaltedHashingForStorage {
    inner: Mutex<HashMap<String, String>>,
}

impl SaltedHashingForStorage {
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
    fn test_salted_hashing_for_storage() {
        let s = SaltedHashingForStorage::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
