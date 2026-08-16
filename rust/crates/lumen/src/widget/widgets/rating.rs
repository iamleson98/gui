//! Star rating widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Rating { style: ResolvedStyle, max: u32, value: f32 }
impl Rating {
    pub fn new(max: u32, value: f32) -> Self { Self { style: Style::new().flex().gap_1().build(), max, value: value.clamp(0.0, max as f32) } }
    pub fn value(&self) -> f32 { self.value }
}

impl Widget for Rating {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Rating" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let sw = rect.width() / self.max as f32;
        for i in 0..self.max {
            let x = rect.min.x + sw * i as f32;
            ctx.painter.fill_rounded_rect(Rect::from_xywh(x, rect.min.y, sw, rect.height()), Color::rgb(226,232,240), crate::style::Corners::all(2.0));
            let filled = (self.value - i as f32).clamp(0.0, 1.0);
            if filled > 0.0 { ctx.painter.fill_rounded_rect(Rect::from_xywh(x, rect.min.y, sw * filled, rect.height()), Color::TW_AMBER_500, crate::style::Corners::all(2.0)); }
        }
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { pos, .. } = event {
            let t = ((pos.x - ctx.current_rect.min.x) / ctx.current_rect.width()).clamp(0.0, 1.0);
            self.value = (t * self.max as f32).ceil();
            ctx.state.request_redraw(); EventResult::HandledAndRedraw
        } else { EventResult::Ignored }
    }
}
