//! Gauge widget.
use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Gauge { style: ResolvedStyle, value: f32, color: Color }
impl Gauge {
    pub fn new(value: f32) -> Self { Self { style: Style::new().w(120.0).h(120.0).build(), value: value.clamp(0.0, 1.0), color: Color::TW_INDIGO_500 } }
    pub fn value(&self) -> f32 { self.value }
}

impl Widget for Gauge {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Gauge" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let cx = rect.center().x; let cy = rect.center().y;
        let r = rect.width().min(rect.height()) * 0.5 - 8.0;
        if r <= 0.0 { return; }
        let start = 3.0 * std::f32::consts::FRAC_PI_4; let sweep = 3.0 * std::f32::consts::FRAC_PI_2;
        const STEPS: usize = 64;
        let fill_end = start + sweep * self.value;
        for i in 0..STEPS {
            let a = start + sweep * i as f32 / STEPS as f32;
            if a > fill_end { break; }
            let p = crate::core::Vec2::new(cx + r * a.cos(), cy + r * a.sin());
            ctx.painter.fill_rect(Rect::from_xywh(p.x - 3.0, p.y - 3.0, 6.0, 6.0), self.color);
        }
    }
}
