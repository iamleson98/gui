//! Removable chip/tag widget.
use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Chip {
    style: ResolvedStyle,
    id: usize,
    label: SmolStr,
    removable: bool,
    selected: bool,
}
impl Chip {
    pub fn new(id: usize, label: impl Into<SmolStr>) -> Self {
        Self {
            style: Style::new()
                .px_3()
                .py_1()
                .rounded_full()
                .bg(Color::rgb(226, 232, 240))
                .text_sm()
                .cursor_pointer()
                .build(),
            id,
            label: label.into(),
            removable: false,
            selected: false,
        }
    }
    pub fn removable(mut self) -> Self {
        self.removable = true;
        self
    }
    pub fn selected(mut self) -> Self {
        self.selected = true;
        self
    }
    pub fn label(&self) -> &str {
        &self.label
    }
}

impl Widget for Chip {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Chip"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let bg = if self.selected {
            Color::TW_INDIGO_500
        } else {
            self.style.background
        };
        ctx.painter
            .fill_rounded_rect(*rect, bg, self.style.border_radius);
    }
}
