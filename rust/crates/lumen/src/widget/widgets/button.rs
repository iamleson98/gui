use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// The visual variant of a button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Solid primary-color button with white text. Use for the main action
    /// in a view.
    #[default]
    Primary,
    /// Outlined button with a border and transparent background. Use for
    /// secondary actions.
    Secondary,
    /// Borderless button with muted text. Use for tertiary actions.
    Ghost,
}

/// A push button with a text label rendered using cosmic-text.
pub struct Button {
    style: ResolvedStyle,
    label: SmolStr,
    variant: ButtonVariant,
    hovered: bool,
    pressed: bool,
    on_click: Option<Box<dyn Fn(&Id) + Send + Sync>>,
}

impl Button {
    /// Create a primary button (solid indigo background, white text).
    pub fn new(label: impl Into<SmolStr>) -> Self {
        Self::with_variant(label, ButtonVariant::Primary)
    }

    /// Create a secondary button (outlined, transparent background).
    pub fn secondary(label: impl Into<SmolStr>) -> Self {
        Self::with_variant(label, ButtonVariant::Secondary)
    }

    /// Create a ghost button (no border, no background).
    pub fn ghost(label: impl Into<SmolStr>) -> Self {
        Self::with_variant(label, ButtonVariant::Ghost)
    }

    /// Create a button with the given variant. Applies the default styling
    /// for that variant: large padding, rounded-lg, medium font weight.
    pub fn with_variant(label: impl Into<SmolStr>, variant: ButtonVariant) -> Self {
        let style = match variant {
            ButtonVariant::Primary => Style::new()
                .px_6()
                .py_3()
                .rounded_lg()
                .bg(Color::TW_INDIGO_600)
                .text_white()
                .text_base()
                .font_medium()
                .cursor_pointer()
                .build(),
            ButtonVariant::Secondary => Style::new()
                .px_6()
                .py_3()
                .rounded_lg()
                .bg(Color::WHITE)
                .border(1.0)
                .border_color(Color::TW_SLATE_300)
                .text_color(Color::TW_SLATE_700)
                .text_base()
                .font_medium()
                .cursor_pointer()
                .build(),
            ButtonVariant::Ghost => Style::new()
                .px_4()
                .py_2()
                .rounded_lg()
                .text_color(Color::TW_SLATE_600)
                .text_base()
                .font_medium()
                .cursor_pointer()
                .build(),
        };
        Self {
            style,
            label: label.into(),
            variant,
            hovered: false,
            pressed: false,
            on_click: None,
        }
    }

    pub fn on_click(mut self, f: impl Fn(&Id) + Send + Sync + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn variant(&self) -> ButtonVariant {
        self.variant
    }
}

impl Widget for Button {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Button"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn text_measure(&self) -> Option<(&str, f32)> {
        Some((&self.label, self.style.font_size))
    }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        // Resolve the current background color based on variant + interaction
        // state.
        let (bg, border) = self.resolved_colors();

        // Draw a subtle drop shadow under primary buttons (not when pressed).
        if matches!(self.variant, ButtonVariant::Primary) && !self.pressed {
            ctx.painter.fill_shadow(
                *rect,
                self.style.border_radius,
                Vec2::new(0.0, 1.0),
                2.0,
                Color::rgba(15, 23, 42, 30),
            );
        }

        // Fill the button background.
        ctx.painter
            .fill_rounded_rect(*rect, bg, self.style.border_radius);

        // Draw the border for secondary buttons.
        if matches!(self.variant, ButtonVariant::Secondary) {
            ctx.painter.stroke_rect(*rect, border, 1.0);
        }

        // Center the text precisely using the measured width.
        let text_size = ctx.text.measure_text(&self.label, self.style.font_size);
        let origin = Vec2::new(
            rect.center().x - text_size.x * 0.5,
            rect.center().y - text_size.y * 0.5,
        );
        let glyphs =
            ctx.text
                .layout_text(&self.label, self.style.font_size, self.style.color, origin);
        for g in &glyphs {
            ctx.painter.push_glyph(g.rect, g.uv, g.color);
        }
    }

    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerMove { .. } => {
                if !self.hovered {
                    self.hovered = true;
                    ctx.state.request_redraw();
                    ctx.state.set_cursor(crate::style::Cursor::Pointer);
                    EventResult::Handled
                } else {
                    EventResult::Ignored
                }
            }
            Event::PointerLeave => {
                if self.hovered || self.pressed {
                    self.hovered = false;
                    self.pressed = false;
                    ctx.state.request_redraw();
                    EventResult::Handled
                } else {
                    EventResult::Ignored
                }
            }
            Event::PointerDown { .. } => {
                self.pressed = true;
                ctx.state.capture_pointer(ctx.current_id);
                EventResult::Handled
            }
            Event::PointerUp { .. } => {
                if self.pressed {
                    self.pressed = false;
                    if let Some(f) = &self.on_click {
                        f(&ctx.current_id);
                    }
                    ctx.state.release_pointer();
                    EventResult::HandledAndRedraw
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Button {
    /// Compute the current (background, border) colors based on variant and
    /// hover/pressed state.
    fn resolved_colors(&self) -> (Color, Color) {
        match self.variant {
            ButtonVariant::Primary => {
                if self.pressed {
                    (Color::TW_INDIGO_700, Color::TRANSPARENT)
                } else if self.hovered {
                    (Color::TW_INDIGO_500, Color::TRANSPARENT)
                } else {
                    (Color::TW_INDIGO_600, Color::TRANSPARENT)
                }
            }
            ButtonVariant::Secondary => {
                let bg = if self.pressed {
                    Color::TW_SLATE_100
                } else if self.hovered {
                    Color::TW_SLATE_50
                } else {
                    Color::WHITE
                };
                let border = if self.hovered {
                    Color::TW_SLATE_400
                } else {
                    Color::TW_SLATE_300
                };
                (bg, border)
            }
            ButtonVariant::Ghost => {
                let bg = if self.pressed {
                    Color::TW_SLATE_100
                } else if self.hovered {
                    Color::TW_SLATE_50
                } else {
                    Color::TRANSPARENT
                };
                (bg, Color::TRANSPARENT)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_default_is_primary() {
        let b = Button::new("Save");
        assert_eq!(b.variant(), ButtonVariant::Primary);
    }

    #[test]
    fn button_secondary_uses_outlined_style() {
        let b = Button::secondary("Cancel");
        assert_eq!(b.variant(), ButtonVariant::Secondary);
        assert!(b.style.border_width.left > 0.0);
    }

    #[test]
    fn button_ghost_has_no_background() {
        let b = Button::ghost("Skip");
        assert_eq!(b.variant(), ButtonVariant::Ghost);
        // Ghost buttons start transparent.
        assert_eq!(b.style.background, Color::TRANSPARENT);
    }

    #[test]
    fn button_primary_has_generous_padding() {
        let b = Button::new("Click");
        // px_6 = 24px, py_3 = 12px
        assert_eq!(b.style.padding.left, 24.0);
        assert_eq!(b.style.padding.top, 12.0);
    }
}
