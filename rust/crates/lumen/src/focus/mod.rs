//! Focus management — Tab navigation, focus ring, focus traps.
//!
//! Manages which widget currently has keyboard focus, and handles
//! Tab/Shift+Tab navigation between focusable widgets.

use crate::core::Id;
use std::collections::HashSet;

/// Manages focus state across the widget tree.
pub struct FocusManager {
    /// The currently focused widget, if any.
    focused: Option<Id>,
    /// All registered focusable widget IDs, in tab order.
    focus_order: Vec<Id>,
    /// If set, focus is trapped within this set (for modals).
    focus_trap: Option<HashSet<Id>>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self {
            focused: None,
            focus_order: Vec::new(),
            focus_trap: None,
        }
    }
}

impl FocusManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a widget as focusable.
    pub fn register(&mut self, id: Id) {
        if !self.focus_order.contains(&id) {
            self.focus_order.push(id);
        }
    }

    /// Unregister a widget.
    pub fn unregister(&mut self, id: Id) {
        self.focus_order.retain(|&i| i != id);
        if self.focused == Some(id) {
            self.focused = None;
        }
    }

    /// Get the currently focused widget.
    pub fn focused(&self) -> Option<Id> {
        self.focused
    }

    /// Set focus to a specific widget.
    pub fn set_focus(&mut self, id: Id) {
        if self.focus_trap.as_ref().map_or(true, |trap| trap.contains(&id)) {
            self.focused = Some(id);
        }
    }

    /// Advance focus to the next focusable widget (Tab key).
    pub fn next(&mut self) {
        if self.focus_order.is_empty() {
            return;
        }
        let visible: Vec<Id> = self.focus_order.iter().filter(|id| {
            self.focus_trap.as_ref().map_or(true, |trap| trap.contains(id))
        }).copied().collect();
        if visible.is_empty() {
            return;
        }
        let new = match self.focused {
            None => visible[0],
            Some(cur) => {
                let pos = visible.iter().position(|&i| i == cur).unwrap_or(0);
                visible[(pos + 1) % visible.len()]
            }
        };
        self.focused = Some(new);
    }

    /// Move focus to the previous focusable widget (Shift+Tab).
    pub fn prev(&mut self) {
        if self.focus_order.is_empty() {
            return;
        }
        let visible: Vec<Id> = self.focus_order.iter().filter(|id| {
            self.focus_trap.as_ref().map_or(true, |trap| trap.contains(id))
        }).copied().collect();
        if visible.is_empty() {
            return;
        }
        let new = match self.focused {
            None => visible[visible.len() - 1],
            Some(cur) => {
                let pos = visible.iter().position(|&i| i == cur).unwrap_or(0);
                visible[if pos == 0 { visible.len() - 1 } else { pos - 1 }]
            }
        };
        self.focused = Some(new);
    }

    /// Clear focus.
    pub fn clear(&mut self) {
        self.focused = None;
    }

    /// Start a focus trap (for modal dialogs).
    pub fn start_trap(&mut self, ids: HashSet<Id>) {
        self.focus_trap = Some(ids);
    }

    /// End the focus trap.
    pub fn end_trap(&mut self) {
        self.focus_trap = None;
    }

    /// Is a widget currently focused?
    pub fn is_focused(&self, id: Id) -> bool {
        self.focused == Some(id)
    }

    /// Number of registered focusable widgets.
    pub fn focusable_count(&self) -> usize {
        self.focus_order.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_focus() {
        let mut fm = FocusManager::new();
        let a = Id::new("a");
        let b = Id::new("b");
        fm.register(a);
        fm.register(b);
        assert_eq!(fm.focusable_count(), 2);
        fm.set_focus(a);
        assert_eq!(fm.focused(), Some(a));
    }

    #[test]
    fn tab_navigation() {
        let mut fm = FocusManager::new();
        let a = Id::new("a");
        let b = Id::new("b");
        let c = Id::new("c");
        fm.register(a);
        fm.register(b);
        fm.register(c);
        fm.next();
        assert_eq!(fm.focused(), Some(a));
        fm.next();
        assert_eq!(fm.focused(), Some(b));
        fm.next();
        assert_eq!(fm.focused(), Some(c));
        fm.next();
        assert_eq!(fm.focused(), Some(a)); // wraps
    }

    #[test]
    fn shift_tab_navigation() {
        let mut fm = FocusManager::new();
        let a = Id::new("a");
        let b = Id::new("b");
        fm.register(a);
        fm.register(b);
        fm.prev();
        assert_eq!(fm.focused(), Some(b)); // starts from last
        fm.prev();
        assert_eq!(fm.focused(), Some(a));
    }

    #[test]
    fn focus_trap() {
        let mut fm = FocusManager::new();
        let a = Id::new("a");
        let b = Id::new("b");
        let c = Id::new("c");
        fm.register(a);
        fm.register(b);
        fm.register(c);
        let mut trap = HashSet::new();
        trap.insert(a);
        trap.insert(b);
        fm.start_trap(trap);
        fm.set_focus(a);
        fm.next();
        assert_eq!(fm.focused(), Some(b));
        fm.next();
        assert_eq!(fm.focused(), Some(a)); // wraps within trap
    }

    #[test]
    fn unregister_removes() {
        let mut fm = FocusManager::new();
        let a = Id::new("a");
        fm.register(a);
        fm.set_focus(a);
        fm.unregister(a);
        assert_eq!(fm.focusable_count(), 0);
        assert_eq!(fm.focused(), None);
    }
}
