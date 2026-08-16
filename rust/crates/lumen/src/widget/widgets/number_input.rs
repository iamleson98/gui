//! Number input widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::input::Modifiers;
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct NumberInput { style: ResolvedStyle, value: f64, min: f64, max: f64, step: f64, text: String, focused: bool }
impl NumberInput {
    pub fn new(value: f64) -> Self { let text = format!("{}", value as i64); Self { style: Style::new().px_2().py_1().rounded_md().bg_white().border(1.0).border_color(Color::rgb(203,213,225)).text_sm().build(), value, min: f64::MIN, max: f64::MAX, step: 1.0, text, focused: false } }
    pub fn with_range(mut self, min: f64, max: f64) -> Self { self.min = min; self.max = max; self.value = self.value.clamp(min, max); self.text = format!("{}", self.value as i64); self }
    pub fn with_step(mut self, step: f64) -> Self { self.step = step; self }
    pub fn value(&self) -> f64 { self.value }
    fn step_value(&mut self, dir: i32, mods: Modifiers, ctx: &mut EventCtx<'_>) {
        let s = if mods.shift() { self.step * 10.0 } else if mods.ctrl() { self.step * 0.1 } else { self.step };
        let new_v = (self.value + dir as f64 * s).clamp(self.min, self.max);
        if (new_v - self.value).abs() > 1e-9 { self.value = new_v; self.text = format!("{}", self.value as i64); ctx.state.request_redraw(); }
    }
}

impl Widget for NumberInput {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "NumberInput" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let border = if self.focused { Color::TW_INDIGO_500 } else { self.style.border_color };
        ctx.painter.stroke_rect(*rect, border, 1.0);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerDown { pos, .. } => {
                let btn_w = 16.0;
                if pos.x >= ctx.current_rect.max.x - btn_w {
                    let up = pos.y < ctx.current_rect.center().y;
                    self.step_value(if up { 1 } else { -1 }, Modifiers::empty(), ctx);
                    EventResult::HandledAndRedraw
                } else { self.focused = true; ctx.state.request_focus(ctx.current_id); EventResult::Handled }
            }
            Event::KeyDown { code, modifiers } => {
                if !self.focused { return EventResult::Ignored; }
                match code {
                    crate::input::KeyCode::ArrowUp => { self.step_value(1, *modifiers, ctx); EventResult::Handled }
                    crate::input::KeyCode::ArrowDown => { self.step_value(-1, *modifiers, ctx); EventResult::Handled }
                    crate::input::KeyCode::Escape => { self.focused = false; EventResult::Handled }
                    _ => EventResult::Ignored,
                }
            }
            _ => EventResult::Ignored,
        }
    }
}
