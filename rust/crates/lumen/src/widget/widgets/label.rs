use crate::core::Rect;
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// A non-interactive text label.
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
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Label" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        if self.style.background.a > 0 {
            ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        }

        // Render text as character blocks
        let char_w = self.style.font_size * 0.6;
        let char_h = self.style.font_size;
        let _total_w = self.text.chars().count() as f32 * char_w;
        let start_x = rect.min.x;
        let start_y = rect.center().y - char_h * 0.5;
        let text_color = self.style.color;
        for (i, _ch) in self.text.chars().enumerate() {
            let char_rect = Rect::from_xywh(
                start_x + i as f32 * char_w,
                start_y,
                char_w * 0.8,
                char_h,
            );
            ctx.painter.fill_rounded_rect(char_rect, text_color, crate::style::Corners::all(1.0));
        }
    }
}
