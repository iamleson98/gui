//! Multi-select dropdown widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct SelectOption { pub label: SmolStr, pub disabled: bool }
impl SelectOption { pub fn new(label: impl Into<SmolStr>) -> Self { Self { label: label.into(), disabled: false } } }

pub struct Select {
    style: ResolvedStyle,
    options: Vec<SelectOption>,
    selected: Vec<usize>,
    open: bool,
}

impl Select {
    pub fn single(options: Vec<SelectOption>) -> Self {
        Self { style: Style::new().px_3().py_2().rounded_md().bg_white().border(1.0).border_color(Color::rgb(203,213,225)).text_sm().cursor_pointer().build(), options, selected: Vec::new(), open: false }
    }
    pub fn multi(options: Vec<SelectOption>) -> Self { Self::single(options) }
    pub fn selected(&self) -> &[usize] { &self.selected }
    pub fn is_open(&self) -> bool { self.open }
}

impl Widget for Select {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Select" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 1.0);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { .. } = event { self.open = !self.open; ctx.state.request_redraw(); EventResult::Handled } else { EventResult::Ignored }
    }
}
