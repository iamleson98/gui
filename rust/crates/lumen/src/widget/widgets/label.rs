use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Label { style: ResolvedStyle, text: SmolStr }
impl Label {
    pub fn new(text: impl Into<SmolStr>) -> Self { Self { style: Style::new().text_sm().build(), text: text.into() } }
    pub fn with_style(mut self, s: ResolvedStyle) -> Self { self.style = s; self }
    pub fn text(&self) -> &str { &self.text }
}
impl Widget for Label {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Label" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { if self.style.background.a > 0 { ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius); } }
}
