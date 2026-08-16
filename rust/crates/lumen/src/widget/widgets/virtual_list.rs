//! Virtualized list — only renders visible rows, supports millions of items.

use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

/// Emitted when the visible range changes.
pub struct VisibleRangeChanged {
    pub start: usize,
    pub end: usize,
}

/// A virtualized list that only measures/paints visible rows.
/// Supports millions of items with O(1) rendering.
pub struct VirtualList {
    style: ResolvedStyle,
    total_items: usize,
    row_height: f32,
    scroll_offset: f32,
    selected: Option<usize>,
    hovered: Option<usize>,
    /// Builder function that creates a widget for a given index.
    /// In a full implementation this would be a closure, but for
    /// simplicity we just paint a colored row.
    on_select: Option<Box<dyn Fn(usize) + Send + Sync>>,
}

impl VirtualList {
    pub fn new(total_items: usize, row_height: f32) -> Self {
        Self {
            style: Style::new().overflow_hidden().bg_white().build(),
            total_items,
            row_height,
            scroll_offset: 0.0,
            selected: None,
            hovered: None,
            on_select: None,
        }
    }

    pub fn total_items(&self) -> usize {
        self.total_items
    }
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }
    pub fn scroll_offset(&self) -> f32 {
        self.scroll_offset
    }

    /// Returns the range of visible item indices.
    pub fn visible_range(&self, viewport_height: f32) -> (usize, usize) {
        if self.total_items == 0 {
            return (0, 0);
        }
        let start = ((self.scroll_offset / self.row_height).floor() as usize)
            .min(self.total_items.saturating_sub(1));
        let visible_count = ((viewport_height / self.row_height).ceil() as usize) + 1;
        let end = (start + visible_count).min(self.total_items);
        (start, end)
    }

    /// Set total items (e.g., after data load).
    pub fn set_total(&mut self, total: usize) {
        self.total_items = total;
        let max_scroll = (total as f32 * self.row_height) - 1.0;
        self.scroll_offset = self.scroll_offset.min(max_scroll.max(0.0));
    }

    /// Scroll to a specific item index.
    pub fn scroll_to(&mut self, index: usize) {
        if index >= self.total_items {
            return;
        }
        self.scroll_offset = (index as f32 * self.row_height).max(0.0);
    }

    pub fn on_select<F: Fn(usize) + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }
}

impl Widget for VirtualList {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "VirtualList"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter
            .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        ctx.painter.push_clip(*rect);

        let (_, _end) = self.visible_range(rect.height());
        let (start, end) = self.visible_range(rect.height());

        for i in start..end {
            let y = rect.min.y + (i as f32 * self.row_height) - self.scroll_offset;
            if y + self.row_height < rect.min.y || y > rect.max.y {
                continue;
            }
            let row_rect = Rect::from_xywh(rect.min.x, y, rect.width(), self.row_height);
            let bg = if self.selected == Some(i) {
                Color::rgba(99, 102, 241, 40)
            } else if self.hovered == Some(i) {
                Color::rgb(241, 245, 249)
            } else {
                Color::TRANSPARENT
            };
            if bg.a > 0 {
                ctx.painter.fill_rect(row_rect, bg);
            }
        }
        ctx.painter.pop_clip();
    }

    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::Scroll { delta, .. } => {
                let dy = delta.y * self.row_height;
                let total_h = self.total_items as f32 * self.row_height;
                let max_scroll = (total_h - ctx.current_rect.height()).max(0.0);
                self.scroll_offset = (self.scroll_offset + dy).clamp(0.0, max_scroll);
                ctx.state.request_redraw();
                EventResult::HandledAndRedraw
            }
            Event::PointerMove { pos, .. } => {
                let rel_y = pos.y - ctx.current_rect.min.y + self.scroll_offset;
                let idx = (rel_y / self.row_height) as usize;
                let new_hover = if idx < self.total_items {
                    Some(idx)
                } else {
                    None
                };
                if new_hover != self.hovered {
                    self.hovered = new_hover;
                    if self.hovered.is_some() {
                        ctx.state.set_cursor(crate::style::Cursor::Pointer);
                    }
                    ctx.state.request_redraw();
                }
                EventResult::Ignored
            }
            Event::PointerLeave => {
                if self.hovered.is_some() {
                    self.hovered = None;
                    ctx.state.request_redraw();
                }
                EventResult::Ignored
            }
            Event::PointerDown { pos, .. } => {
                let rel_y = pos.y - ctx.current_rect.min.y + self.scroll_offset;
                if rel_y >= 0.0 {
                    let idx = (rel_y / self.row_height) as usize;
                    if idx < self.total_items {
                        self.selected = Some(idx);
                        if let Some(f) = &self.on_select {
                            f(idx);
                        }
                        ctx.state.request_redraw();
                        return EventResult::HandledAndRedraw;
                    }
                }
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_range_basic() {
        let vl = VirtualList::new(1000, 30.0);
        let (start, end) = vl.visible_range(300.0);
        assert_eq!(start, 0);
        assert!(end > start);
        assert!(end <= 11); // ~10 visible + 1 buffer
    }

    #[test]
    fn visible_range_with_scroll() {
        let mut vl = VirtualList::new(1000, 30.0);
        vl.scroll_offset = 300.0; // scrolled past 10 items
        let (start, end) = vl.visible_range(300.0);
        assert!(start >= 10);
        assert!(end > start);
    }

    #[test]
    fn scroll_to() {
        let mut vl = VirtualList::new(1000, 30.0);
        vl.scroll_to(500);
        assert!((vl.scroll_offset - 15000.0).abs() < 0.1);
    }

    #[test]
    fn set_total_clamps_scroll() {
        let mut vl = VirtualList::new(1000, 30.0);
        vl.scroll_offset = 20000.0;
        vl.set_total(100);
        assert!(vl.scroll_offset <= 100.0 * 30.0 - 1.0);
    }

    #[test]
    fn empty_list() {
        let vl = VirtualList::new(0, 30.0);
        let (start, end) = vl.visible_range(300.0);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
    }
}
