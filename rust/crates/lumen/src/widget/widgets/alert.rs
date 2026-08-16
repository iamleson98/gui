//! Alert banner widget.
use crate::core::{Color, Rect};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

#[derive(Clone, Copy)]
pub enum AlertKind {
    Info,
    Success,
    Warning,
    Error,
}
impl AlertKind {
    pub fn colors(self) -> (Color, Color) {
        match self {
            Self::Info => (Color::rgb(219, 234, 254), Color::TW_INDIGO_500),
            Self::Success => (Color::rgb(209, 250, 229), Color::TW_EMERALD_500),
            Self::Warning => (Color::rgb(254, 249, 195), Color::TW_AMBER_500),
            Self::Error => (Color::rgb(254, 226, 226), Color::TW_ROSE_500),
        }
    }
}

pub struct Alert {
    style: ResolvedStyle,
    kind: AlertKind,
}
impl Alert {
    pub fn new(kind: AlertKind, _title: impl Into<SmolStr>, _msg: impl Into<SmolStr>) -> Self {
        let (bg, _border) = kind.colors();
        Self {
            style: Style::new()
                .px_4()
                .py_3()
                .rounded_md()
                .bg(bg)
                .border(1.0)
                .build(),
            kind,
        }
    }
}

impl Widget for Alert {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn debug_name(&self) -> &'static str {
        "Alert"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let (bg, border) = AlertKind::colors(self.kind);
        ctx.painter
            .fill_rounded_rect(*rect, bg, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, border, 2.0);
    }
}
