//! Simple dropdown widget.
use crate::core::Rect;
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Dropdown {
    style: ResolvedStyle,
    options: Vec<SmolStr>,
    selected: Option<usize>,
    open: bool,
}

impl Dropdown {
    pub fn new(options: Vec<impl Into<SmolStr>>, selected: usize) -> Self {
        let o: Vec<SmolStr> = options.into_iter().map(Into::into).collect();
        Self {
            style: Style::new()
                .px_3()
                .py_2()
                .rounded_md()
                .bg_white()
                .border(1.0)
                .cursor_pointer()
                .text_sm()
                .build(),
            selected: if selected < o.len() {
                Some(selected)
            } else {
                None
            },
            options: o,
            open: false,
        }
    }
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }
}

impl Widget for Dropdown {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Dropdown"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter
            .fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 1.0);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { .. } = event {
            self.open = !self.open;
            ctx.state.request_redraw();
            EventResult::Handled
        } else {
            EventResult::Ignored
        }
    }
}
