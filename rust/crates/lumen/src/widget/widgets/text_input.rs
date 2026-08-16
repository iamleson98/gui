use crate::core::{Color, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// Emitted when the text content changes.
pub struct TextChanged {
    pub text: SmolStr,
}

/// Emitted when Enter is pressed.
pub struct TextSubmitted {
    pub text: SmolStr,
}

/// A single-line editable text field with cosmic-text rendering.
pub struct TextInput {
    style: ResolvedStyle,
    text: SmolStr,
    placeholder: SmolStr,
    focused: bool,
}

impl TextInput {
    pub fn new(placeholder: impl Into<SmolStr>) -> Self {
        Self {
            style: Style::new()
                .px_3()
                .py_2()
                .rounded_md()
                .bg_white()
                .border(1.0)
                .border_color(Color::rgb(203, 213, 225))
                .text_sm()
                .build(),
            text: SmolStr::new(""),
            placeholder: placeholder.into(),
            focused: false,
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

    fn render_text(&self, ctx: &mut PaintCtx<'_>, text: &str, origin: Vec2, color: Color) {
        let glyphs = ctx
            .text
            .layout_text(text, self.style.font_size, color, origin);
        for g in &glyphs {
            ctx.painter.push_glyph(g.rect, g.uv, g.color);
        }
    }
}

impl Widget for TextInput {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "TextInput"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn text_measure(&self) -> Option<(&str, f32)> {
        // Measure the placeholder when empty so the input still has a size.
        let text = if self.text.is_empty() {
            &self.placeholder
        } else {
            &self.text
        };
        Some((text, self.style.font_size))
    }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter
            .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let border = if self.focused {
            Color::TW_INDIGO_500
        } else {
            self.style.border_color
        };
        ctx.painter.stroke_rect(*rect, border, 1.0);

        let text_size = ctx
            .text
            .measure_text(&self.text, self.style.font_size);
        let origin = Vec2::new(
            rect.min.x + 8.0,
            rect.center().y - text_size.y * 0.5,
        );

        if !self.text.is_empty() {
            self.render_text(ctx, &self.text, origin, self.style.color);
        } else if !self.placeholder.is_empty() {
            self.render_text(ctx, &self.placeholder, origin, Color::rgb(148, 163, 184));
        }

        // Render caret when focused
        if self.focused {
            // Use the measured text width so the caret sits right after
            // the last typed character.
            let caret_x = rect.min.x + 8.0 + text_size.x;
            ctx.painter.fill_rect(
                Rect::from_xywh(caret_x, rect.min.y + 6.0, 2.0, rect.height() - 12.0),
                Color::TW_INDIGO_500,
            );
        }
    }

    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerDown { .. } => {
                self.focused = true;
                ctx.state.request_focus(ctx.current_id);
                ctx.state.request_redraw();
                EventResult::Handled
            }
            Event::FocusLost => {
                if self.focused {
                    self.focused = false;
                    ctx.state.request_redraw();
                    EventResult::Handled
                } else {
                    EventResult::Ignored
                }
            }
            Event::Char { c } => {
                if self.focused && !c.is_control() {
                    let mut s = self.text.to_string();
                    s.push(*c);
                    self.text = SmolStr::new(&s);
                    ctx.state.emit(TextChanged {
                        text: self.text.clone(),
                    });
                    ctx.state.request_redraw();
                    EventResult::Handled
                } else {
                    EventResult::Ignored
                }
            }
            Event::KeyDown { code, .. } => {
                if !self.focused {
                    return EventResult::Ignored;
                }
                match code {
                    crate::input::KeyCode::Backspace => {
                        if !self.text.is_empty() {
                            let mut s = self.text.to_string();
                            let nl = s.char_indices().last().map(|(i, _)| i).unwrap_or(0);
                            s.truncate(nl);
                            self.text = SmolStr::new(&s);
                            ctx.state.emit(TextChanged {
                                text: self.text.clone(),
                            });
                            ctx.state.request_redraw();
                        }
                        EventResult::Handled
                    }
                    crate::input::KeyCode::Enter => {
                        ctx.state.emit(TextSubmitted {
                            text: self.text.clone(),
                        });
                        EventResult::Handled
                    }
                    crate::input::KeyCode::Escape => {
                        self.focused = false;
                        ctx.state.request_redraw();
                        EventResult::Handled
                    }
                    _ => EventResult::Ignored,
                }
            }
            _ => EventResult::Ignored,
        }
    }
}
