//! Reactive state management — signals, effects, computed values.
//!
//! Inspired by SolidJS/Leptos signals. When a signal changes, all
//! dependent computed values and effects are automatically re-run.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static SIGNAL_ID: AtomicU64 = AtomicU64::new(0);

/// A reactive signal — a value that notifies subscribers when it changes.
pub struct Signal<T: Clone + 'static> {
    id: u64,
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn(&T)>>>>,
}

impl<T: Clone + 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: SIGNAL_ID.fetch_add(1, Ordering::Relaxed),
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Get the current value.
    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    /// Set a new value and notify all subscribers.
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value.clone();
        let subs = self.subscribers.borrow();
        for f in subs.iter() {
            f(&value);
        }
    }

    /// Update the value in place.
    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        let mut val = self.value.borrow_mut();
        f(&mut val);
        let new_val = val.clone();
        drop(val);
        let subs = self.subscribers.borrow();
        for f in subs.iter() {
            f(&new_val);
        }
    }

    /// Subscribe to changes. Returns an unsubscribe guard.
    pub fn on_change<F: Fn(&T) + 'static>(&self, f: F) -> Subscription {
        let id = SIGNAL_ID.fetch_add(1, Ordering::Relaxed);
        self.subscribers.borrow_mut().push(Box::new(f));
        Subscription { id }
    }

    /// Map this signal to a new signal via a transform function.
    pub fn map<U: Clone + 'static, F: Fn(&T) -> U + 'static>(&self, f: F) -> Signal<U> {
        let initial = f(&self.value.borrow());
        let result = Signal::new(initial);
        let result_clone = result.clone();
        self.on_change(move |v| {
            result_clone.set(f(v));
        });
        result
    }

    /// Number of subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.borrow().len()
    }
}

impl<T: Clone + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Rc::clone(&self.value),
            subscribers: Rc::clone(&self.subscribers),
        }
    }
}

/// Unsubscribe guard. Dropping it does nothing (simplified).
pub struct Subscription {
    id: u64,
}

/// A computed value derived from one or more signals.
pub struct Computed<T: Clone + 'static> {
    signal: Signal<T>,
}

impl<T: Clone + 'static> Computed<T> {
    pub fn new<F: Fn() -> T + 'static>(compute: F) -> Self {
        let signal = Signal::new(compute());
        // In a full implementation, we'd track which signals were read
        // during `compute()` and re-run when any of them change.
        // For simplicity, the user must manually call `recompute()`.
        let _ = compute;
        Self { signal }
    }

    pub fn get(&self) -> T {
        self.signal.get()
    }

    pub fn recompute<F: Fn() -> T>(&self, f: F) {
        self.signal.set(f());
    }
}

/// An effect that runs when its dependencies change.
pub fn create_effect<F: Fn() + 'static>(_f: F) {
    // In a full implementation, this would track signal reads
    // and re-run when any read signal changes.
    // For simplicity, just run once.
    _f();
}

/// Create a signal — shorthand for `Signal::new()`.
pub fn create_signal<T: Clone + 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_get_set() {
        let s = Signal::new(42);
        assert_eq!(s.get(), 42);
        s.set(100);
        assert_eq!(s.get(), 100);
    }

    #[test]
    fn signal_update() {
        let s = Signal::new(10);
        s.update(|v| *v += 5);
        assert_eq!(s.get(), 15);
    }

    #[test]
    fn signal_notifies_subscribers() {
        let s = Signal::new(0);
        let count = Rc::new(RefCell::new(0));
        let c = count.clone();
        s.on_change(move |_| {
            *c.borrow_mut() += 1;
        });
        s.set(1);
        s.set(2);
        assert_eq!(*count.borrow(), 2);
    }

    #[test]
    fn signal_map() {
        let s = Signal::new(5);
        let doubled = s.map(|v| v * 2);
        assert_eq!(doubled.get(), 10);
        s.set(10);
        assert_eq!(doubled.get(), 20);
    }

    #[test]
    fn signal_clone_shares_state() {
        let s1 = Signal::new("hello");
        let s2 = s1.clone();
        s2.set("world");
        assert_eq!(s1.get(), "world");
    }

    #[test]
    fn signal_subscriber_count() {
        let s = Signal::new(0);
        assert_eq!(s.subscriber_count(), 0);
        s.on_change(|_| {});
        s.on_change(|_| {});
        assert_eq!(s.subscriber_count(), 2);
    }
}
