use crate::core::{Color, Id, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

/// A push button with a text label.
pub struct Button {
    style: ResolvedStyle,
    label: SmolStr,
    hovered: bool,
    pressed: bool,
    on_click: Option<Box<dyn Fn(&Id) + Send + Sync>>,
}

impl Button {
    pub fn new(label: impl Into<SmolStr>) -> Self {
        Self {
            style: Style::new()
                .px_4().py_2()
                .rounded_md()
                .bg_primary()
                .text_white()
                .font_medium()
                .cursor_pointer()
                .build(),
            label: label.into(),
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
}

impl Widget for Button {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Button" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let mut bg = self.style.background;
        if self.pressed {
            bg = bg.lerp(Color::BLACK, 0.15);
        } else if self.hovered {
            bg = bg.lerp(Color::WHITE, 0.10);
        }
        ctx.painter.fill_rounded_rect(*rect, bg, self.style.border_radius);

        // Render text as character blocks
        let char_w = self.style.font_size * 0.6;
        let char_h = self.style.font_size;
        let total_w = self.label.chars().count() as f32 * char_w;
        let start_x = rect.center().x - total_w * 0.5;
        let start_y = rect.center().y - char_h * 0.5;
        let text_color = self.style.color;
        for (i, _ch) in self.label.chars().enumerate() {
            let char_rect = Rect::from_xywh(
                start_x + i as f32 * char_w,
                start_y,
                char_w * 0.8,
                char_h,
            );
            ctx.painter.fill_rounded_rect(char_rect, text_color, crate::style::Corners::all(1.0));
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
