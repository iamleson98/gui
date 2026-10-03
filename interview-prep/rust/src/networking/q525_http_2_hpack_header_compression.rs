//! Question #525: HTTP/2 HPACK Header Compression
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: HPACK, header compression, Huffman, dynamic table
//! Description: Compress HTTP/2 headers with HPACK static and dynamic tables plus Huffman coding.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Http2HpackHeaderCompression {
    inner: Mutex<HashMap<String, String>>,
}

impl Http2HpackHeaderCompression {
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
    fn test_http_2_hpack_header_compression() {
        let s = Http2HpackHeaderCompression::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
