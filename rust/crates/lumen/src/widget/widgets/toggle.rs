use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Toggle { style: ResolvedStyle, on: bool }
impl Toggle { pub fn new(on: bool) -> Self { Self { style: Style::new().w(44.0).h(24.0).rounded_full().bg(Color::rgb(203,213,225)).cursor_pointer().build(), on } } pub fn is_on(&self) -> bool { self.on } }
impl Widget for Toggle {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Toggle" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let track = if self.on { Color::TW_EMERALD_500 } else { Color::rgb(203,213,225) };
        ctx.painter.fill_rounded_rect(*rect, track, self.style.border_radius);
        let r = rect.height() * 0.4; let travel = rect.width() - r * 2.0 - 4.0;
        let x = rect.min.x + 2.0 + r + travel * (if self.on { 1.0 } else { 0.0 });
        let y = rect.center().y;
        ctx.painter.fill_rounded_rect(Rect::from_xywh(x-r, y-r, r*2.0, r*2.0), Color::WHITE, crate::style::Corners::all(r));
    }
    fn on_event(&mut self, _ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerUp { .. } = event { self.on = !self.on; EventResult::HandledAndRedraw } else if let Event::PointerDown { .. } = event { EventResult::Handled } else { EventResult::Ignored }
    }
}
