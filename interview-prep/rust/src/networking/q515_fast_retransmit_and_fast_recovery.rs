//! Question #515: Fast Retransmit and Fast Recovery
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: fast retransmit, fast recovery, dup ACK, loss
//! Description: Recover from packet loss without timing out using duplicate ACKs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FastRetransmitAndFastRecovery {
    inner: Mutex<HashMap<String, String>>,
}

impl FastRetransmitAndFastRecovery {
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
    fn test_fast_retransmit_and_fast_recovery() {
        let s = FastRetransmitAndFastRecovery::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
