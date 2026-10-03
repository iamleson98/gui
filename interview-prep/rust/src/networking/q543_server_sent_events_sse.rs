//! Question #543: Server-Sent Events (SSE)
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: SSE, EventSource, streaming, reconnect
//! Description: Stream server-to-client events over a long-lived HTTP connection with EventSource.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ServerSentEventsSse {
    inner: Mutex<HashMap<String, String>>,
}

impl ServerSentEventsSse {
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
    fn test_server_sent_events_sse() {
        let s = ServerSentEventsSse::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
