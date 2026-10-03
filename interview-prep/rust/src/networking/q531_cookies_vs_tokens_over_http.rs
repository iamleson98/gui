//! Question #531: Cookies vs Tokens over HTTP
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: cookies, tokens, CORS, CSRF
//! Description: Contrast cookie-based and token-based authentication over HTTP and their tradeoffs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CookiesVsTokensOverHttp {
    inner: Mutex<HashMap<String, String>>,
}

impl CookiesVsTokensOverHttp {
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
    fn test_cookies_vs_tokens_over_http() {
        let s = CookiesVsTokensOverHttp::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
