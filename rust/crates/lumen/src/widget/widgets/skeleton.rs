//! Skeleton loader widget.
use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Skeleton { style: ResolvedStyle, progress: f32 }
impl Skeleton {
    pub fn new() -> Self { Self { style: Style::new().w_full().h(16.0).rounded_md().bg(Color::rgb(226,232,240)).build(), progress: 0.0 } }
    pub fn tick(&mut self, dt: f32) { self.progress = (self.progress + dt * 0.5) % 1.0; }
}

impl Default for Skeleton { fn default() -> Self { Self::new() } }
impl Widget for Skeleton {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Skeleton" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let sw = rect.width() * 0.3; let sx = rect.min.x + (rect.width() + sw) * self.progress - sw * 0.5;
        let sr = Rect::from_xywh(sx, rect.min.y, sw, rect.height()).intersect(*rect);
        if sr.width() > 0.0 { ctx.painter.fill_rounded_rect(sr, Color::rgba(255,255,255,80), self.style.border_radius); }
    }
}
