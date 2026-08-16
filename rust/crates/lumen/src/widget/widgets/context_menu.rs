//! Context menu — right-click popup menu.

use crate::core::{Color, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::input::MouseButton;
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// Emitted when a context menu item is selected.
pub struct ContextMenuSelected {
    pub index: usize,
}

/// A context menu item.
pub struct ContextMenuItem {
    pub label: SmolStr,
    pub disabled: bool,
    pub separator: bool,
}

impl ContextMenuItem {
    pub fn new(label: impl Into<SmolStr>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            separator: false,
        }
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    pub fn separator() -> Self {
        Self {
            label: SmolStr::new(""),
            disabled: true,
            separator: true,
        }
    }
}

/// A context menu that appears on right-click.
pub struct ContextMenu {
    style: ResolvedStyle,
    items: Vec<ContextMenuItem>,
    open: bool,
    position: Vec2,
    hovered: Option<usize>,
    on_select: Option<Box<dyn Fn(usize) + Send + Sync>>,
}

impl ContextMenu {
    pub fn new(items: Vec<ContextMenuItem>) -> Self {
        Self {
            style: Style::new().bg_white().rounded_md().shadow_md().build(),
            items,
            open: false,
            position: Vec2::ZERO,
            hovered: None,
            on_select: None,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn close(&mut self) {
        self.open = false;
        self.hovered = None;
    }

    pub fn on_select<F: Fn(usize) + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }
}

impl Widget for ContextMenu {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "ContextMenu"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn paint(&self, ctx: &mut PaintCtx<'_>, _rect: &Rect) {
        if !self.open {
            return;
        }
        let item_h = 32.0;
        let menu_w = 180.0;
        let menu_h = self.items.len() as f32 * item_h;
        let menu_rect = Rect::from_xywh(self.position.x, self.position.y, menu_w, menu_h);
        ctx.painter
            .fill_rounded_rect(menu_rect, self.style.background, self.style.border_radius);
        ctx.painter
            .stroke_rect(menu_rect, Color::rgb(203, 213, 225), 1.0);
        for (i, item) in self.items.iter().enumerate() {
            let y = self.position.y + i as f32 * item_h;
            if item.separator {
                ctx.painter.fill_rect(
                    Rect::from_xywh(menu_rect.min.x, y + item_h * 0.5, menu_w, 1.0),
                    Color::rgb(226, 232, 240),
                );
                continue;
            }
            if self.hovered == Some(i) && !item.disabled {
                ctx.painter.fill_rect(
                    Rect::from_xywh(menu_rect.min.x, y, menu_w, item_h),
                    Color::TW_INDIGO_500,
                );
            }
        }
    }

    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerDown { pos, button, .. } => {
                if *button == MouseButton::Right {
                    self.position = *pos;
                    self.open = true;
                    ctx.state.request_redraw();
                    return EventResult::Handled;
                }
                if *button == MouseButton::Left && self.open {
                    let item_h = 32.0;
                    let menu_w = 180.0;
                    let menu_rect = Rect::from_xywh(
                        self.position.x,
                        self.position.y,
                        menu_w,
                        self.items.len() as f32 * item_h,
                    );
                    if menu_rect.contains(*pos) {
                        let idx = ((pos.y - self.position.y) / item_h) as usize;
                        if idx < self.items.len()
                            && !self.items[idx].disabled
                            && !self.items[idx].separator
                        {
                            if let Some(f) = &self.on_select {
                                f(idx);
                            }
                            ctx.state.emit(ContextMenuSelected { index: idx });
                            self.close();
                            return EventResult::HandledAndRedraw;
                        }
                    } else {
                        self.close();
                        return EventResult::Handled;
                    }
                }
                EventResult::Ignored
            }
            Event::PointerMove { pos, .. } => {
                if !self.open {
                    return EventResult::Ignored;
                }
                let item_h = 32.0;
                let idx = ((pos.y - self.position.y) / item_h) as usize;
                let new_hover = if idx < self.items.len() && !self.items[idx].separator {
                    Some(idx)
                } else {
                    None
                };
                if new_hover != self.hovered {
                    self.hovered = new_hover;
                    ctx.state.request_redraw();
                }
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}
