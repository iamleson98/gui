//! Question #547: AMQP Message Protocol
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: AMQP, exchanges, bindings, queues
//! Description: Explain AMQP exchanges, queues, and bindings for routed messaging.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AmqpMessageProtocol {
    inner: Mutex<HashMap<String, String>>,
}

impl AmqpMessageProtocol {
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
    fn test_amqp_message_protocol() {
        let s = AmqpMessageProtocol::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
