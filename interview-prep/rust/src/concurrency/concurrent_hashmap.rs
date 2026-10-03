//! Question #34: Concurrent Hash Map with Lock Striping
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: lock striping, segments, concurrent resize

use std::collections::HashMap;
use std::sync::RwLock;

pub struct ConcurrentMap<K: Eq + std::hash::Hash + Clone, V: Clone> {
    shards: Vec<RwLock<HashMap<K, V>>>,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> ConcurrentMap<K, V> {
    pub fn new(shards: usize) -> Self {
        let shards = shards.max(1).next_power_of_two();
        let mut v = Vec::with_capacity(shards);
        for _ in 0..shards {
            v.push(RwLock::new(HashMap::new()));
        }
        Self { shards: v }
    }

    fn idx(&self, key: &K) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (self.shards.len() - 1)
    }

    pub fn put(&self, key: K, value: V) {
        let i = self.idx(&key);
        self.shards[i].write().unwrap().insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let i = self.idx(key);
        self.shards[i].read().unwrap().get(key).cloned()
    }

    pub fn delete(&self, key: &K) -> bool {
        let i = self.idx(key);
        self.shards[i].write().unwrap().remove(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.shards.iter().map(|s| s.read().unwrap().len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let m = ConcurrentMap::new(32);
        m.put("a", 1);
        m.put("b", 2);
        assert_eq!(m.get(&"a"), Some(1));
        m.delete(&"b");
        assert_eq!(m.get(&"b"), None);
        assert_eq!(m.len(), 1);
    }
}
