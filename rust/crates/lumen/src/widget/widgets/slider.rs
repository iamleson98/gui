use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Slider { style: ResolvedStyle, min: f32, max: f32, value: f32, dragging: bool }
impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self { Self { style: Style::new().h(8.0).w_full().rounded_md().bg(Color::rgb(226,232,240)).cursor_pointer().build(), min, max, value: value.clamp(min, max), dragging: false } }
    pub fn value(&self) -> f32 { self.value }
}
impl Widget for Slider {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Slider" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let t = (self.value - self.min) / (self.max - self.min).max(1e-6);
        ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width() * t, rect.height()), Color::TW_INDIGO_500, self.style.border_radius);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { pos, .. } = event { self.dragging = true; ctx.state.capture_pointer(ctx.current_id); let t = ((pos.x - ctx.current_rect.min.x) / ctx.current_rect.width()).clamp(0.0, 1.0); self.value = self.min + t * (self.max - self.min); EventResult::HandledAndRedraw }
        else if let Event::PointerUp { .. } = event { if self.dragging { self.dragging = false; ctx.state.release_pointer(); EventResult::HandledAndRedraw } else { EventResult::Ignored } }
        else { EventResult::Ignored }
    }
}
