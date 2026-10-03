//! Question #545: Thrift and Avro
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: Thrift, Avro, serialization, RPC
//! Description: Contrast Thrift and Avro serialization and RPC frameworks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ThriftAndAvro {
    inner: Mutex<HashMap<String, String>>,
}

impl ThriftAndAvro {
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
    fn test_thrift_and_avro() {
        let s = ThriftAndAvro::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
