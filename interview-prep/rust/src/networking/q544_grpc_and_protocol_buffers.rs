//! Question #544: gRPC and Protocol Buffers
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: gRPC, protobuf, HTTP/2, streaming
//! Description: Design gRPC services with protobuf IDL, streaming, and HTTP/2 transport.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct GrpcAndProtocolBuffers {
    inner: Mutex<HashMap<String, String>>,
}

impl GrpcAndProtocolBuffers {
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
    fn test_grpc_and_protocol_buffers() {
        let s = GrpcAndProtocolBuffers::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
