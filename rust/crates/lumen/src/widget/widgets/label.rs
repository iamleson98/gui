use crate::core::{Rect, Vec2};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// A non-interactive text label rendered using cosmic-text.
pub struct Label {
    style: ResolvedStyle,
    text: SmolStr,
}

impl Label {
    pub fn new(text: impl Into<SmolStr>) -> Self {
        Self {
            style: Style::new().text_sm().build(),
            text: text.into(),
        }
    }

    pub fn with_style(mut self, s: ResolvedStyle) -> Self {
        self.style = s;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, t: impl Into<SmolStr>) {
        self.text = t.into();
    }
}

impl Widget for Label {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Label"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        if self.style.background.a > 0 {
            ctx.painter
                .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        }

        // Render text using cosmic-text
        let origin = Vec2::new(rect.min.x, rect.center().y - self.style.font_size * 0.5);
        let glyphs =
            ctx.text
                .layout_text(&self.text, self.style.font_size, self.style.color, origin);
        for g in &glyphs {
            ctx.painter.push_glyph(g.rect, g.uv, g.color);
        }
    }
}
