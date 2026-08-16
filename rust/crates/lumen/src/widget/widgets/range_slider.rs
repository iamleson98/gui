//! Range slider widget (two thumbs).
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct RangeSlider { style: ResolvedStyle, min: f32, max: f32, start: f32, end: f32 }
impl RangeSlider {
    pub fn new(min: f32, max: f32, start: f32, end: f32) -> Self {
        Self { style: Style::new().h(8.0).w_full().rounded_full().bg(Color::rgb(226,232,240)).cursor_pointer().build(),
            min, max, start: start.clamp(min, max), end: end.clamp(start, max) }
    }
    pub fn start(&self) -> f32 { self.start }
    pub fn end(&self) -> f32 { self.end }
}

impl Widget for RangeSlider {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "RangeSlider" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let st = (self.start - self.min) / (self.max - self.min).max(1e-6);
        let en = (self.end - self.min) / (self.max - self.min).max(1e-6);
        ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x + rect.width() * st, rect.min.y, rect.width() * (en - st), rect.height()), Color::TW_INDIGO_500, self.style.border_radius);
    }
}
