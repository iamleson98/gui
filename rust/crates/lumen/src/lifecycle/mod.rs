//! Widget lifecycle hooks — mount, unmount, update.
//!
//! Inspired by React/Flutter lifecycle methods.
//! Widgets can implement these to run setup/cleanup code.

use crate::core::Id;
use std::collections::HashMap;

/// Lifecycle events that a widget can receive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleEvent {
    /// Widget was added to the tree for the first time.
    Mounted,
    /// Widget's props/state changed and it was re-rendered.
    Updated,
    /// Widget was removed from the tree.
    Unmounted,
    /// Widget became visible (entered viewport).
    BecameVisible,
    /// Widget became invisible (left viewport).
    BecameHidden,
}

/// A lifecycle callback.
pub type LifecycleCallback = Box<dyn Fn(LifecycleEvent) + Send + Sync>;

/// Manages lifecycle events for all widgets.
pub struct LifecycleManager {
    /// Registered callbacks per widget.
    callbacks: HashMap<Id, Vec<LifecycleCallback>>,
    /// Set of mounted widget IDs.
    mounted: HashMap<Id, bool>,
    /// Set of visible widget IDs.
    visible: HashMap<Id, bool>,
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self {
            callbacks: HashMap::new(),
            mounted: HashMap::new(),
            visible: HashMap::new(),
        }
    }
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a lifecycle callback for a widget.
    pub fn on_lifecycle<F: Fn(LifecycleEvent) + Send + Sync + 'static>(
        &mut self,
        id: Id,
        f: F,
    ) {
        self.callbacks.entry(id).or_default().push(Box::new(f));
    }

    /// Called when a widget is added to the tree.
    pub fn mount(&mut self, id: Id) {
        if !self.mounted.get(&id).copied().unwrap_or(false) {
            self.mounted.insert(id, true);
            self.fire(id, LifecycleEvent::Mounted);
        }
    }

    /// Called when a widget is removed from the tree.
    pub fn unmount(&mut self, id: Id) {
        if self.mounted.get(&id).copied().unwrap_or(false) {
            self.mounted.insert(id, false);
            self.fire(id, LifecycleEvent::Unmounted);
            self.callbacks.remove(&id);
            self.mounted.remove(&id);
            self.visible.remove(&id);
        }
    }

    /// Called when a widget is updated.
    pub fn update(&mut self, id: Id) {
        if self.mounted.get(&id).copied().unwrap_or(false) {
            self.fire(id, LifecycleEvent::Updated);
        }
    }

    /// Set widget visibility.
    pub fn set_visible(&mut self, id: Id, visible: bool) {
        let was_visible = self.visible.get(&id).copied().unwrap_or(false);
        if was_visible != visible {
            self.visible.insert(id, visible);
            self.fire(id, if visible { LifecycleEvent::BecameVisible } else { LifecycleEvent::BecameHidden });
        }
    }

    /// Is a widget currently mounted?
    pub fn is_mounted(&self, id: Id) -> bool {
        self.mounted.get(&id).copied().unwrap_or(false)
    }

    /// Is a widget currently visible?
    pub fn is_visible(&self, id: Id) -> bool {
        self.visible.get(&id).copied().unwrap_or(false)
    }

    /// Number of mounted widgets.
    pub fn mounted_count(&self) -> usize {
        self.mounted.values().filter(|&&v| v).count()
    }

    /// Fire a lifecycle event to a widget's callbacks.
    fn fire(&self, id: Id, event: LifecycleEvent) {
        if let Some(callbacks) = self.callbacks.get(&id) {
            for cb in callbacks {
                cb(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[test]
    fn mount_fires_event() {
        let mut lm = LifecycleManager::new();
        let id = Id::new("w1");
        let count = Arc::new(AtomicU32::new(0));
        let c = count.clone();
        lm.on_lifecycle(id, move |e| {
            if e == LifecycleEvent::Mounted {
                c.fetch_add(1, Ordering::Relaxed);
            }
        });
        lm.mount(id);
        assert_eq!(count.load(Ordering::Relaxed), 1);
        assert!(lm.is_mounted(id));
    }

    #[test]
    fn double_mount_noop() {
        let mut lm = LifecycleManager::new();
        let id = Id::new("w1");
        let count = Arc::new(AtomicU32::new(0));
        let c = count.clone();
        lm.on_lifecycle(id, move |e| {
            if e == LifecycleEvent::Mounted {
                c.fetch_add(1, Ordering::Relaxed);
            }
        });
        lm.mount(id);
        lm.mount(id); // should not fire again
        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn unmount_fires_and_cleans_up() {
        let mut lm = LifecycleManager::new();
        let id = Id::new("w1");
        let count = Arc::new(AtomicU32::new(0));
        let c = count.clone();
        lm.on_lifecycle(id, move |e| {
            if e == LifecycleEvent::Unmounted {
                c.fetch_add(1, Ordering::Relaxed);
            }
        });
        lm.mount(id);
        lm.unmount(id);
        assert_eq!(count.load(Ordering::Relaxed), 1);
        assert!(!lm.is_mounted(id));
    }

    #[test]
    fn visibility_changes() {
        let mut lm = LifecycleManager::new();
        let id = Id::new("w1");
        let events = Arc::new(AtomicU32::new(0));
        let e = events.clone();
        lm.on_lifecycle(id, move |evt| {
            if evt == LifecycleEvent::BecameVisible || evt == LifecycleEvent::BecameHidden {
                e.fetch_add(1, Ordering::Relaxed);
            }
        });
        lm.mount(id);
        lm.set_visible(id, true);
        assert!(lm.is_visible(id));
        lm.set_visible(id, false);
        assert!(!lm.is_visible(id));
        assert_eq!(events.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn mounted_count() {
        let mut lm = LifecycleManager::new();
        lm.mount(Id::new("a"));
        lm.mount(Id::new("b"));
        assert_eq!(lm.mounted_count(), 2);
        lm.unmount(Id::new("a"));
        assert_eq!(lm.mounted_count(), 1);
    }
}
