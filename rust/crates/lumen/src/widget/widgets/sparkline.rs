//! Sparkline chart widget.
use crate::core::{Color, Rect, Vec2};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Sparkline {
    style: ResolvedStyle,
    data: Vec<f32>,
    color: Color,
    range_min: f32,
    range_max: f32,
}
impl Sparkline {
    pub fn new() -> Self {
        Self {
            style: Style::new().w_full().h(32.0).build(),
            data: Vec::new(),
            color: Color::TW_INDIGO_500,
            range_min: 0.0,
            range_max: 1.0,
        }
    }
    pub fn push(&mut self, v: f32) {
        self.data.push(v);
        self.recompute();
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.range_min = 0.0;
        self.range_max = 1.0;
    }
    fn recompute(&mut self) {
        if self.data.is_empty() {
            return;
        }
        let mn = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let mx = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if (mx - mn).abs() < 1e-6 {
            self.range_max = mx + 1.0;
            self.range_min = mn - 1.0;
        } else {
            self.range_min = mn;
            self.range_max = mx;
        }
    }
}

impl Default for Sparkline {
    fn default() -> Self {
        Self::new()
    }
}
impl Widget for Sparkline {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Sparkline"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let n = self.data.len();
        if n == 0 {
            return;
        }
        let point = |i: usize| -> Vec2 {
            let tx = if n > 1 {
                i as f32 / (n - 1) as f32
            } else {
                0.5
            };
            let ty = ((self.data[i] - self.range_min)
                / (self.range_max - self.range_min).max(1e-6))
            .clamp(0.0, 1.0);
            Vec2::new(
                rect.min.x + rect.width() * tx,
                rect.min.y + rect.height() * (1.0 - ty),
            )
        };
        if n == 1 {
            let p = point(0);
            ctx.painter
                .fill_rect(Rect::from_xywh(p.x - 2.0, p.y - 2.0, 4.0, 4.0), self.color);
            return;
        }
        let mut prev = point(0);
        for i in 1..n {
            let p = point(i);
            ctx.painter
                .fill_rect(Rect::from_corners(prev, p), self.color);
            prev = p;
        }
    }
}
