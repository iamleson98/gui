//! Single-line text input widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct TextInput {
    style: ResolvedStyle,
    text: SmolStr,
    placeholder: SmolStr,
    focused: bool,
}

impl TextInput {
    pub fn new(placeholder: impl Into<SmolStr>) -> Self {
        Self {
            style: Style::new().px_3().py_2().rounded_md().bg_white().border(1.0).border_color(Color::rgb(203,213,225)).text_sm().build(),
            text: SmolStr::new(""),
            placeholder: placeholder.into(),
            focused: false,
        }
    }
    pub fn with_style(mut self, s: ResolvedStyle) -> Self { self.style = s; self }
    pub fn text(&self) -> &str { &self.text }
    pub fn set_text(&mut self, t: impl Into<SmolStr>) { self.text = t.into(); }
}

impl Widget for TextInput {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "TextInput" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let border = if self.focused { Color::TW_INDIGO_500 } else { self.style.border_color };
        ctx.painter.stroke_rect(*rect, border, 1.0);

        // Render text content as character blocks
        if !self.text.is_empty() {
            let char_w = self.style.font_size * 0.6;
            let char_h = self.style.font_size;
            let start_x = rect.min.x + 8.0;
            let start_y = rect.center().y - char_h * 0.5;
            for (i, _ch) in self.text.chars().enumerate() {
                let char_rect = Rect::from_xywh(
                    start_x + i as f32 * char_w,
                    start_y,
                    char_w * 0.8,
                    char_h,
                );
                ctx.painter.fill_rounded_rect(char_rect, self.style.color, crate::style::Corners::all(1.0));
            }
        }
        // Render caret when focused
        if self.focused {
            let caret_x = rect.min.x + 8.0 + self.text.chars().count() as f32 * self.style.font_size * 0.6;
            ctx.painter.fill_rect(
                Rect::from_xywh(caret_x, rect.min.y + 6.0, 2.0, rect.height() - 12.0),
                Color::TW_INDIGO_500,
            );
        }
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerDown { .. } => { self.focused = true; ctx.state.request_focus(ctx.current_id); ctx.state.request_redraw(); EventResult::Handled }
            Event::FocusLost => { if self.focused { self.focused = false; ctx.state.request_redraw(); EventResult::Handled } else { EventResult::Ignored } }
            Event::Char { c } => {
                if self.focused && !c.is_control() {
                    let mut s = self.text.to_string(); s.push(*c); self.text = SmolStr::new(&s);
                    ctx.state.request_redraw(); EventResult::Handled
                } else { EventResult::Ignored }
            }
            Event::KeyDown { code, .. } => {
                if !self.focused { return EventResult::Ignored; }
                match code {
                    crate::input::KeyCode::Backspace => {
                        if !self.text.is_empty() {
                            let mut s = self.text.to_string();
                            let nl = s.char_indices().last().map(|(i,_)| i).unwrap_or(0);
                            s.truncate(nl); self.text = SmolStr::new(&s);
                            ctx.state.request_redraw();
                        }
                        EventResult::Handled
                    }
                    crate::input::KeyCode::Escape => { self.focused = false; ctx.state.request_redraw(); EventResult::Handled }
                    _ => EventResult::Ignored,
                }
            }
            _ => EventResult::Ignored,
        }
    }
}
