//! Tooltip widget.
use crate::core::Id;
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style};
use crate::widget::{Element, Widget};
use smol_str::SmolStr;

pub struct Tooltip {
    style: ResolvedStyle,
    child: Box<Element>,
    text: SmolStr,
    visible: bool,
    hover_ms: u32,
}
impl Tooltip {
    pub fn new(child: impl Widget + 'static, text: impl Into<SmolStr>) -> Self {
        Self {
            style: Style::default().build(),
            child: Box::new(Element::new(Id::new("tt"), child)),
            text: text.into(),
            visible: false,
            hover_ms: 0,
        }
    }
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Widget for Tooltip {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn children(&self) -> &[Element] {
        std::slice::from_ref(&self.child)
    }
    fn children_mut(&mut self) -> &mut [Element] {
        std::slice::from_mut(&mut *self.child)
    }
    fn debug_name(&self) -> &'static str {
        "Tooltip"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerMove { pos } = event {
            if ctx.current_rect.contains(*pos) {
                self.hover_ms = self.hover_ms.saturating_add(16);
                if !self.visible && self.hover_ms >= 500 {
                    self.visible = true;
                    ctx.state.request_redraw();
                }
                return EventResult::Handled;
            } else if self.visible || self.hover_ms > 0 {
                self.visible = false;
                self.hover_ms = 0;
                ctx.state.request_redraw();
            }
        }
        if let Event::PointerLeave = event {
            if self.visible || self.hover_ms > 0 {
                self.visible = false;
                self.hover_ms = 0;
                ctx.state.request_redraw();
            }
        }
        EventResult::Ignored
    }
}
