//! Clipboard support — copy, cut, paste.

use std::sync::{Arc, Mutex};

/// A simple in-memory clipboard (platform-native would use X11/Win32/Cocoa).
pub struct Clipboard {
    content: Arc<Mutex<String>>,
}

impl Default for Clipboard {
    fn default() -> Self {
        Self {
            content: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl Clipboard {
    pub fn new() -> Self {
        Self::default()
    }

    /// Copy text to the clipboard.
    pub fn copy(&self, text: &str) {
        *self.content.lock().unwrap() = text.to_string();
    }

    /// Get the current clipboard content.
    pub fn paste(&self) -> String {
        self.content.lock().unwrap().clone()
    }

    /// Clear the clipboard.
    pub fn clear(&self) {
        *self.content.lock().unwrap() = String::new();
    }

    /// Check if clipboard has content.
    pub fn has_content(&self) -> bool {
        !self.content.lock().unwrap().is_empty()
    }
}

/// Global clipboard instance.
static GLOBAL_CLIPBOARD: once_cell::sync::Lazy<Clipboard> = once_cell::sync::Lazy::new(Clipboard::default);

/// Copy text to the global clipboard.
pub fn copy(text: &str) {
    GLOBAL_CLIPBOARD.copy(text);
}

/// Paste from the global clipboard.
pub fn paste() -> String {
    GLOBAL_CLIPBOARD.paste()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_paste() {
        let cb = Clipboard::new();
        cb.copy("hello world");
        assert_eq!(cb.paste(), "hello world");
    }

    #[test]
    fn clear() {
        let cb = Clipboard::new();
        cb.copy("data");
        assert!(cb.has_content());
        cb.clear();
        assert!(!cb.has_content());
    }

    #[test]
    fn overwrite() {
        let cb = Clipboard::new();
        cb.copy("first");
        cb.copy("second");
        assert_eq!(cb.paste(), "second");
    }

    #[test]
    fn global_clipboard() {
        copy("test123");
        assert_eq!(paste(), "test123");
    }
}
