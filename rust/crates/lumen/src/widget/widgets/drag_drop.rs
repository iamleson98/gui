//! Drag and drop framework — drag sources, drop targets, data transfer.

use crate::core::{Id, Rect, Vec2};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Type-erased drag data.
pub type DragData = Arc<dyn Any + Send + Sync>;

/// Manages drag and drop state across the widget tree.
pub struct DragDropManager {
    /// Currently dragged data, if any.
    drag_data: Option<DragData>,
    /// Source widget of the current drag.
    drag_source: Option<Id>,
    /// Current pointer position during drag.
    drag_pos: Vec2,
    /// Registered drop targets.
    drop_targets: HashMap<Id, Rect>,
    /// Currently hovered drop target.
    hover_target: Option<Id>,
}

impl Default for DragDropManager {
    fn default() -> Self {
        Self {
            drag_data: None,
            drag_source: None,
            drag_pos: Vec2::ZERO,
            drop_targets: HashMap::new(),
            hover_target: None,
        }
    }
}

impl DragDropManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a drag operation.
    pub fn start_drag(&mut self, source: Id, data: DragData, pos: Vec2) {
        self.drag_data = Some(data);
        self.drag_source = Some(source);
        self.drag_pos = pos;
    }

    /// Update drag position.
    pub fn update_drag(&mut self, pos: Vec2) {
        self.drag_pos = pos;
        self.hover_target = self
            .drop_targets
            .iter()
            .find(|(_, rect)| rect.contains(pos))
            .map(|(id, _)| *id);
    }

    /// End the drag operation. Returns (source, target, data) if dropped on a target.
    pub fn end_drag(&mut self) -> Option<(Id, Id, DragData)> {
        let data = self.drag_data.take()?;
        let source = self.drag_source.take()?;
        let target = self.hover_target.take()?;
        Some((source, target, data))
    }

    /// Is a drag currently in progress?
    pub fn is_dragging(&self) -> bool {
        self.drag_data.is_some()
    }

    /// Register a drop target.
    pub fn register_target(&mut self, id: Id, rect: Rect) {
        self.drop_targets.insert(id, rect);
    }

    /// Get the currently hovered drop target.
    pub fn hover_target(&self) -> Option<Id> {
        self.hover_target
    }

    /// Get drag data as a specific type.
    pub fn get_data<T: 'static + Clone>(&self) -> Option<T> {
        self.drag_data.as_ref()?.downcast_ref::<T>().cloned()
    }
}

/// A widget that can be dragged.
pub struct Draggable {
    /// The data to transfer when dragged.
    data: Option<DragData>,
    /// Whether a drag is currently in progress from this widget.
    dragging: bool,
    /// Callback when drag starts.
    on_drag_start: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Draggable {
    pub fn new(data: DragData) -> Self {
        Self {
            data: Some(data),
            dragging: false,
            on_drag_start: None,
        }
    }

    pub fn on_drag_start<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_drag_start = Some(Box::new(f));
        self
    }
}

/// A widget that accepts drops.
pub struct DropTarget {
    /// Whether this target is currently being hovered.
    hovered: bool,
    /// Callback when something is dropped.
    on_drop: Option<Box<dyn Fn(&DragData) + Send + Sync>>,
}

impl DropTarget {
    pub fn new() -> Self {
        Self {
            hovered: false,
            on_drop: None,
        }
    }

    pub fn on_drop<F: Fn(&DragData) + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_drop = Some(Box::new(f));
        self
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }
}

impl Default for DropTarget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_end_drag() {
        let mut dnd = DragDropManager::new();
        let source = Id::new("src");
        let data: DragData = Arc::new(42i32);
        dnd.start_drag(source, data.clone(), Vec2::new(10.0, 10.0));
        assert!(dnd.is_dragging());
        dnd.update_drag(Vec2::new(50.0, 50.0));
        let result = dnd.end_drag();
        assert!(result.is_none()); // No drop target registered
    }

    #[test]
    fn drop_on_target() {
        let mut dnd = DragDropManager::new();
        let source = Id::new("src");
        let target = Id::new("target");
        let data: DragData = Arc::new("hello".to_string());
        dnd.register_target(target, Rect::from_xywh(40.0, 40.0, 20.0, 20.0));
        dnd.start_drag(source, data, Vec2::new(10.0, 10.0));
        dnd.update_drag(Vec2::new(50.0, 50.0));
        assert_eq!(dnd.hover_target(), Some(target));
        let result = dnd.end_drag();
        assert!(result.is_some());
        let (s, t, _) = result.unwrap();
        assert_eq!(s, source);
        assert_eq!(t, target);
    }

    #[test]
    fn get_typed_data() {
        let mut dnd = DragDropManager::new();
        let data: DragData = Arc::new(42i32);
        dnd.start_drag(Id::new("src"), data, Vec2::ZERO);
        assert_eq!(dnd.get_data::<i32>(), Some(42));
        assert_eq!(dnd.get_data::<String>(), None);
    }
}
