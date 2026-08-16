use crate::core::{Color, Rect, Vec2};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};

/// A surface container with a subtle border, drop shadow, and rounded
/// corners. Use to group related content into a visual section.
///
/// Children are laid out in a vertical flex column with a 12px gap.
pub struct Card {
    style: ResolvedStyle,
    body: Vec<Element>,
}

impl Card {
    /// Create a card with the given children. The card has a white
    /// background, 1px slate-200 border, rounded-xl corners (14px), 24px
    /// padding, and a subtle drop shadow.
    pub fn new(body: Vec<Element>) -> Self {
        Self {
            style: Style::new()
                .p_6()
                .rounded_xl()
                .bg(Color::WHITE)
                .border(1.0)
                .border_color(Color::TW_SLATE_200)
                .flex_col()
                .gap_4()
                .items_stretch()
                .build(),
            body,
        }
    }

    /// Create a card with custom padding and a header.
    pub fn with_padding(body: Vec<Element>, padding: f32) -> Self {
        let mut card = Self::new(body);
        card.style.padding = crate::style::Edges::all(padding);
        card
    }

    pub fn with_style(mut self, s: ResolvedStyle) -> Self {
        self.style = s;
        self
    }
}

impl Widget for Card {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn children(&self) -> &[Element] {
        &self.body
    }
    fn children_mut(&mut self) -> &mut [Element] {
        &mut self.body
    }
    fn debug_name(&self) -> &'static str {
        "Card"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        // Subtle drop shadow underneath the card.
        ctx.painter.fill_shadow(
            *rect,
            self.style.border_radius,
            Vec2::new(0.0, 2.0),
            4.0,
            Color::rgba(15, 23, 42, 24),
        );
        // Card background.
        ctx.painter
            .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        // 1px border.
        if self.style.border_width.left > 0.0 {
            ctx.painter
                .stroke_rect(*rect, self.style.border_color, self.style.border_width.left);
        }
    }
}
