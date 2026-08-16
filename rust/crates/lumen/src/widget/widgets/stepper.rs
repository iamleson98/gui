//! Stepper / wizard widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Step { pub label: SmolStr, pub optional: bool }
impl Step { pub fn new(label: impl Into<SmolStr>) -> Self { Self { label: label.into(), optional: false } } }

pub struct Stepper { style: ResolvedStyle, steps: Vec<Step>, current: usize }
impl Stepper {
    pub fn new(steps: Vec<Step>) -> Self { Self { style: Style::new().flex().build(), steps, current: 0 } }
    pub fn current(&self) -> usize { self.current }
}

impl Widget for Stepper {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Stepper" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let n = self.steps.len(); if n == 0 { return; }
        let sw = rect.width() / n as f32; let r = 14.0;
        for (i, _) in self.steps.iter().enumerate() {
            let cx = rect.min.x + sw * (i as f32 + 0.5); let cy = rect.min.y + r + 4.0;
            let bg = if i < self.current { Color::TW_EMERALD_500 } else if i == self.current { Color::TW_INDIGO_500 } else { Color::rgb(203,213,225) };
            ctx.painter.fill_rounded_rect(Rect::from_xywh(cx - r, cy - r, r * 2.0, r * 2.0), bg, crate::style::Corners::all(r));
        }
    }
}
