use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Progress { style: ResolvedStyle, value: f32 }
impl Progress { pub fn new(value: f32) -> Self { Self { style: Style::new().h(8.0).w_full().rounded_md().bg(Color::rgb(226,232,240)).build(), value: value.clamp(0.0, 1.0) } } }
impl Widget for Progress {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Progress" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width() * self.value, rect.height()), Color::TW_INDIGO_500, self.style.border_radius);
    }
}
