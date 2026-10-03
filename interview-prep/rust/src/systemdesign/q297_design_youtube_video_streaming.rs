//! Question #297: Design YouTube / Video Streaming
//! Category: System Design | Difficulty: Hard
//! Concepts: video streaming, adaptive bitrate, transcoding, CDN
//! Description: Design a video platform with adaptive bitrate, transcoding pipelines, and CDN.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignYoutubeVideoStreaming {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignYoutubeVideoStreaming {
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
    fn test_design_youtube_video_streaming() {
        let s = DesignYoutubeVideoStreaming::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
