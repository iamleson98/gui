//! Error boundaries — catch panics in widget rendering and show fallback UI.
//!
//! If a widget panics during paint or event handling, the error boundary
//! catches it and displays a fallback instead of crashing the entire app.

use crate::core::Id;
use std::any::Any;
use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe, UnwindSafe};
use std::rc::Rc;

/// Result of running a fallible operation.
pub type FallibleResult<T> = Result<T, Box<dyn Any + Send>>;

/// Catch panics from a closure and return a Result.
pub fn catch_panics<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> FallibleResult<R> {
    panic::catch_unwind(f)
}

/// An error boundary that wraps a widget subtree.
pub struct ErrorBoundary {
    /// Whether the boundary has caught an error.
    has_error: bool,
    /// The error message if any.
    error_message: Option<String>,
    /// Number of errors caught.
    error_count: u32,
    /// Callback when an error is caught.
    on_error: Option<Box<dyn Fn(&str)>>,
}

impl Default for ErrorBoundary {
    fn default() -> Self {
        Self {
            has_error: false,
            error_message: None,
            error_count: 0,
            on_error: None,
        }
    }
}

impl ErrorBoundary {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a callback for when an error occurs.
    pub fn on_error<F: Fn(&str) + 'static>(&mut self, f: F) {
        self.on_error = Some(Box::new(f));
    }

    /// Run a fallible operation. If it panics, catch it and record the error.
    pub fn try_run<F: FnOnce() -> R + UnwindSafe, R>(&mut self, f: F) -> Option<R> {
        match catch_panics(f) {
            Ok(result) => Some(result),
            Err(e) => {
                self.has_error = true;
                self.error_count += 1;
                let msg = if let Some(s) = e.downcast_ref::<&'static str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };
                self.error_message = Some(msg.clone());
                if let Some(f) = &self.on_error {
                    f(&msg);
                }
                None
            }
        }
    }

    /// Whether the boundary has caught an error.
    pub fn has_error(&self) -> bool {
        self.has_error
    }

    /// Get the last error message.
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Number of errors caught.
    pub fn error_count(&self) -> u32 {
        self.error_count
    }

    /// Clear the error state (retry).
    pub fn reset(&mut self) {
        self.has_error = false;
        self.error_message = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catch_successful() {
        let mut eb = ErrorBoundary::new();
        let result = eb.try_run(|| 42);
        assert_eq!(result, Some(42));
        assert!(!eb.has_error());
    }

    #[test]
    fn catch_panic_str() {
        let mut eb = ErrorBoundary::new();
        let result = eb.try_run(|| {
            panic!("widget crashed");
        });
        assert_eq!(result, None);
        assert!(eb.has_error());
        assert_eq!(eb.error_count(), 1);
    }

    #[test]
    fn catch_panic_string() {
        let mut eb = ErrorBoundary::new();
        let _ = eb.try_run(|| {
            panic!("{}", "formatted error");
        });
        assert!(eb.has_error());
        assert!(eb.error_message().unwrap().contains("formatted error"));
    }

    #[test]
    fn reset_clears_error() {
        let mut eb = ErrorBoundary::new();
        let _ = eb.try_run(|| panic!("oops"));
        assert!(eb.has_error());
        eb.reset();
        assert!(!eb.has_error());
    }

    #[test]
    fn error_callback_fired() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        let called = Arc::new(AtomicBool::new(false));
        let c = called.clone();
        let mut eb = ErrorBoundary::new();
        eb.on_error(move |_| { c.store(true, Ordering::Relaxed); });
        let _ = eb.try_run(|| panic!("test"));
        assert!(called.load(Ordering::Relaxed));
    }

    #[test]
    fn multiple_errors_counted() {
        let mut eb = ErrorBoundary::new();
        let _ = eb.try_run(|| panic!("1"));
        let _ = eb.try_run(|| panic!("2"));
        let _ = eb.try_run(|| panic!("3"));
        assert_eq!(eb.error_count(), 3);
    }
}
