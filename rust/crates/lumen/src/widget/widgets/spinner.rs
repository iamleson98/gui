//! Loading spinner widget.
use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Spinner {
    style: ResolvedStyle,
    size: f32,
    angle: f32,
    color: Color,
}
impl Spinner {
    pub fn new(size: f32) -> Self {
        Self {
            style: Style::new().w(size).h(size).build(),
            size,
            angle: 0.0,
            color: Color::TW_INDIGO_500,
        }
    }
    pub fn tick(&mut self, dt: f32) {
        self.angle = (self.angle + dt * std::f32::consts::TAU) % std::f32::consts::TAU;
    }
}

impl Widget for Spinner {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Spinner"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let cx = rect.center().x;
        let cy = rect.center().y;
        let r = self.size * 0.5 - 3.0;
        if r <= 0.0 {
            return;
        }
        const SEG: usize = 12;
        for i in 0..SEG {
            let t0 = i as f32 / SEG as f32;
            let a = self.angle + t0 * std::f32::consts::TAU;
            let alpha = ((1.0 - t0).max(0.1) * 255.0) as u8;
            let px = cx + r * a.cos();
            let py = cy + r * a.sin();
            ctx.painter.fill_rect(
                Rect::from_xywh(px - 2.0, py - 2.0, 4.0, 4.0),
                self.color.with_alpha(alpha),
            );
        }
    }
}
