use crate::core::{Color, Rect, Vec2};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// The visual variant of a label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelVariant {
    /// Large, bold heading text (28px).
    Heading,
    /// Medium bold subheading (18px).
    Subheading,
    /// Default body text (14px).
    #[default]
    Body,
    /// Smaller muted text (12px), e.g. captions / helper text.
    Caption,
}

/// A non-interactive text label rendered using cosmic-text.
pub struct Label {
    style: ResolvedStyle,
    text: SmolStr,
    variant: LabelVariant,
}

impl Label {
    /// Create a default body-text label (14px, slate-900).
    pub fn new(text: impl Into<SmolStr>) -> Self {
        Self::with_variant(text, LabelVariant::Body)
    }

    /// Create a large heading label (28px, bold).
    pub fn heading(text: impl Into<SmolStr>) -> Self {
        Self::with_variant(text, LabelVariant::Heading)
    }

    /// Create a medium subheading label (18px, bold).
    pub fn subheading(text: impl Into<SmolStr>) -> Self {
        Self::with_variant(text, LabelVariant::Subheading)
    }

    /// Create a small muted caption label (12px).
    pub fn caption(text: impl Into<SmolStr>) -> Self {
        Self::with_variant(text, LabelVariant::Caption)
    }

    /// Create a label with the given variant and its default styling.
    pub fn with_variant(text: impl Into<SmolStr>, variant: LabelVariant) -> Self {
        let style = match variant {
            LabelVariant::Heading => Style::new()
                .text_2xl()
                .font_bold()
                .text_color(Color::TW_SLATE_900)
                .build(),
            LabelVariant::Subheading => Style::new()
                .text_lg()
                .font_bold()
                .text_color(Color::TW_SLATE_700)
                .build(),
            LabelVariant::Body => Style::new()
                .text_base()
                .text_color(Color::TW_SLATE_900)
                .build(),
            LabelVariant::Caption => Style::new()
                .text_sm()
                .text_color(Color::TW_SLATE_500)
                .build(),
        };
        Self {
            style,
            text: text.into(),
            variant,
        }
    }

    pub fn with_style(mut self, s: ResolvedStyle) -> Self {
        self.style = s;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn variant(&self) -> LabelVariant {
        self.variant
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
    fn text_measure(&self) -> Option<(&str, f32)> {
        Some((&self.text, self.style.font_size))
    }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        if self.style.background.a > 0 {
            ctx.painter
                .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        }

        // Vertically center the text inside the rect.
        let text_size = ctx.text.measure_text(&self.text, self.style.font_size);
        let origin = Vec2::new(rect.min.x, rect.center().y - text_size.y * 0.5);
        let glyphs =
            ctx.text
                .layout_text(&self.text, self.style.font_size, self.style.color, origin);
        for g in &glyphs {
            ctx.painter.push_glyph(g.rect, g.uv, g.color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_default_is_body() {
        let l = Label::new("Hello");
        assert_eq!(l.variant(), LabelVariant::Body);
        assert_eq!(l.style.font_size, 14.0);
    }

    #[test]
    fn label_heading_is_large_bold() {
        let l = Label::heading("Title");
        assert_eq!(l.variant(), LabelVariant::Heading);
        assert_eq!(l.style.font_size, 28.0);
        assert_eq!(l.style.font_weight, 700);
    }

    #[test]
    fn label_caption_is_small_muted() {
        let l = Label::caption("Helper text");
        assert_eq!(l.variant(), LabelVariant::Caption);
        assert_eq!(l.style.font_size, 12.0);
        assert_eq!(l.style.color, Color::TW_SLATE_500);
    }
}
