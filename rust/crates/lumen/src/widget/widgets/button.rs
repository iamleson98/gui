use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Button { style: ResolvedStyle, label: SmolStr, hovered: bool, pressed: bool, on_click: Option<Box<dyn Fn(&Id) + Send + Sync>> }
impl Button {
    pub fn new(label: impl Into<SmolStr>) -> Self {
        Self { style: Style::new().px_4().py_2().rounded_md().bg_primary().text_white().font_medium().cursor_pointer().build(), label: label.into(), hovered: false, pressed: false, on_click: None }
    }
    pub fn on_click(mut self, f: impl Fn(&Id) + Send + Sync + 'static) -> Self { self.on_click = Some(Box::new(f)); self }
}
impl Widget for Button {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Button" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let mut c = self.style.background;
        if self.pressed { c = c.lerp(Color::BLACK, 0.15); } else if self.hovered { c = c.lerp(Color::WHITE, 0.10); }
        ctx.painter.fill_rounded_rect(*rect, c, self.style.border_radius);
        let _ = (&self.label, ctx);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerMove { .. } => { if !self.hovered { self.hovered = true; ctx.state.request_redraw(); ctx.state.set_cursor(crate::style::Cursor::Pointer); EventResult::Handled } else { EventResult::Ignored } }
            Event::PointerLeave => { if self.hovered || self.pressed { self.hovered = false; self.pressed = false; ctx.state.request_redraw(); EventResult::Handled } else { EventResult::Ignored } }
            Event::PointerDown { .. } => { self.pressed = true; ctx.state.capture_pointer(ctx.current_id); EventResult::Handled }
            Event::PointerUp { .. } => { if self.pressed { self.pressed = false; if let Some(f) = &self.on_click { f(&ctx.current_id); } ctx.state.release_pointer(); EventResult::HandledAndRedraw } else { EventResult::Ignored } }
            _ => EventResult::Ignored,
        }
    }
}
