use crate::core::Rect;
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};

pub struct Card { style: ResolvedStyle, body: Vec<Element> }
impl Card { pub fn new(body: Vec<Element>) -> Self { Self { style: Style::new().p_4().rounded_lg().bg_white().shadow_md().flex_col().gap_2().build(), body } } }
impl Widget for Card {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn children(&self) -> &[Element] { &self.body }
    fn children_mut(&mut self) -> &mut [Element] { &mut self.body }
    fn debug_name(&self) -> &'static str { "Card" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius); }
}
