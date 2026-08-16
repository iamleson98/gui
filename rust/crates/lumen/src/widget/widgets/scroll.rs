use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Scroll { style: ResolvedStyle, children: Vec<Element>, offset: Vec2 }
impl Scroll { pub fn new(children: Vec<Element>) -> Self { Self { style: Style::new().overflow_hidden().rounded_md().bg(Color::TRANSPARENT).build(), children, offset: Vec2::ZERO } } }
impl Widget for Scroll {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn children(&self) -> &[Element] { &self.children }
    fn children_mut(&mut self) -> &mut [Element] { &mut self.children }
    fn debug_name(&self) -> &'static str { "Scroll" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { if self.style.background.a > 0 { ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius); } ctx.painter.push_clip(*rect); ctx.painter.translate(Vec2::new(-self.offset.x, -self.offset.y)); }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::Scroll { delta, .. } = event { self.offset.y = (self.offset.y + delta.y * 20.0).max(0.0); EventResult::HandledAndRedraw } else { EventResult::Ignored }
    }
}
