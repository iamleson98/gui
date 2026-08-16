//! Modal dialog widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};

pub struct Dialog { style: ResolvedStyle, content: Vec<Element>, is_open: bool }
impl Dialog {
    pub fn new(content: Vec<Element>) -> Self { Self { style: Style::new().p_4().rounded_lg().bg_white().shadow_lg().max_w(480.0).build(), content, is_open: false } }
    pub fn is_open(&self) -> bool { self.is_open }
    pub fn open_dialog(&mut self) { self.is_open = true; }
    pub fn close(&mut self) { self.is_open = false; }
}

impl Widget for Dialog {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn children(&self) -> &[Element] { if self.is_open { &self.content } else { &[] } }
    fn children_mut(&mut self) -> &mut [Element] { if self.is_open { &mut self.content } else { &mut [] } }
    fn debug_name(&self) -> &'static str { "Dialog" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        if !self.is_open { return; }
        ctx.painter.fill_rect(*rect, Color::rgba(0,0,0,120));
        let panel = Rect::from_xywh(rect.center().x - 240.0, rect.center().y - 100.0, 480.0, 200.0);
        ctx.painter.fill_rounded_rect(panel, self.style.background, self.style.border_radius);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if !self.is_open { return EventResult::Ignored; }
        if let Event::PointerDown { .. } = event { self.is_open = false; ctx.state.request_redraw(); EventResult::Handled }
        else if let Event::KeyDown { code, .. } = event { if *code == crate::input::KeyCode::Escape { self.is_open = false; ctx.state.request_redraw(); EventResult::Handled } else { EventResult::Ignored } }
        else { EventResult::Ignored }
    }
}
