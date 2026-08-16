//! Color picker widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct ColorPicker { style: ResolvedStyle, color: Color, open: bool }
impl ColorPicker {
    pub fn new(color: Color) -> Self { Self { style: Style::new().w(64.0).h(32.0).rounded_md().border(1.0).border_color(Color::rgb(203,213,225)).cursor_pointer().build(), color, open: false } }
    pub fn color(&self) -> Color { self.color }
}

impl Widget for ColorPicker {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "ColorPicker" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.color, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 1.0);
    }
}
