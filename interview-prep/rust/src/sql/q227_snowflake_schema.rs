//! Question #227: Snowflake Schema
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: snowflake, normalized dimensions, OLAP, storage
//! Description: Normalize dimensions in a star schema to form a snowflake and weigh query vs storage tradeoffs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SnowflakeSchema {
    inner: Mutex<HashMap<String, String>>,
}

impl SnowflakeSchema {
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
    fn test_snowflake_schema() {
        let s = SnowflakeSchema::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
