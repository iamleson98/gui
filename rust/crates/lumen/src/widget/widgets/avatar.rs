use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Avatar {
    style: ResolvedStyle,
    bg: Color,
}
impl Avatar {
    pub fn new(initials: impl Into<SmolStr>, size: f32) -> Self {
        let i: SmolStr = initials.into();
        let bg = avatar_color(&i);
        Self {
            style: Style::new().w(size).h(size).rounded_full().bg(bg).build(),
            bg,
        }
    }
}
impl Widget for Avatar {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Avatar"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter
            .fill_rounded_rect(*rect, self.bg, self.style.border_radius);
    }
}
pub fn avatar_color(s: &str) -> Color {
    use std::hash::{Hash, Hasher};
    let palette = [
        Color::TW_INDIGO_500,
        Color::TW_EMERALD_500,
        Color::TW_ROSE_500,
        Color::TW_AMBER_500,
    ];
    let mut h = ahash::AHasher::default();
    s.hash(&mut h);
    palette[(h.finish() as usize) % palette.len()]
}
