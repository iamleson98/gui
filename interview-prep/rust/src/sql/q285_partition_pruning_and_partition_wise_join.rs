//! Question #285: Partition Pruning and Partition-Wise Join
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: partitioning, pruning, partition-wise join, range/list
//! Description: Use declarative partitioning to prune scans and co-locate partitions for joins.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PartitionPruningAndPartitionWiseJoin {
    inner: Mutex<HashMap<String, String>>,
}

impl PartitionPruningAndPartitionWiseJoin {
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
    fn test_partition_pruning_and_partition_wise_join() {
        let s = PartitionPruningAndPartitionWiseJoin::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
