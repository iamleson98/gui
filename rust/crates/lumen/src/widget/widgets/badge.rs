use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Badge { style: ResolvedStyle, color: Color }
impl Badge { pub fn new(_text: impl Into<SmolStr>, color: Color) -> Self { Self { style: Style::new().px_2().py_1().rounded_full().build(), color } } }
impl Widget for Badge {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Badge" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { ctx.painter.fill_rounded_rect(*rect, self.color, self.style.border_radius); }
}
