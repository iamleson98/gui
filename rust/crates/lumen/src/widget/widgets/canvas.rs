use crate::core::Rect;
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Canvas {
    style: ResolvedStyle,
}
impl Canvas {
    pub fn new() -> Self {
        Self {
            style: Style::new().w_full().h_full().bg_white().build(),
        }
    }
}
impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
impl Widget for Canvas {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Canvas"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter
            .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
    }
}
