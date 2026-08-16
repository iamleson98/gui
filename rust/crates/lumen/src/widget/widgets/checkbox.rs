use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Checkbox { style: ResolvedStyle, checked: bool, hovered: bool }
impl Checkbox {
    pub fn new(checked: bool) -> Self { Self { style: Style::new().w(18.0).h(18.0).rounded_md().border(2.0).border_color(Color::rgb(148,163,184)).bg_white().cursor_pointer().build(), checked, hovered: false } }
    pub fn checked(&self) -> bool { self.checked }
}
impl Widget for Checkbox {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Checkbox" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let bg = if self.checked { Color::TW_INDIGO_500 } else if self.hovered { Color::rgb(241,245,249) } else { self.style.background };
        ctx.painter.fill_rounded_rect(*rect, bg, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 2.0);
        if self.checked { ctx.painter.fill_rect(rect.inset(4.0), Color::WHITE); }
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerDown { .. } => { ctx.state.capture_pointer(ctx.current_id); EventResult::Handled }
            Event::PointerUp { .. } => { self.checked = !self.checked; ctx.state.release_pointer(); EventResult::HandledAndRedraw }
            _ => EventResult::Ignored,
        }
    }
}
