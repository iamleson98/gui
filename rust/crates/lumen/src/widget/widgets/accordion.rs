//! Accordion widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct AccordionSection { pub header: SmolStr, pub content: Vec<Element>, pub open: bool, pub disabled: bool }
impl AccordionSection { pub fn new(header: impl Into<SmolStr>, content: Vec<Element>) -> Self { Self { header: header.into(), content, open: false, disabled: false } } pub fn open(mut self) -> Self { self.open = true; self } }

pub struct Accordion { style: ResolvedStyle, sections: Vec<AccordionSection> }
impl Accordion {
    pub fn new(sections: Vec<AccordionSection>) -> Self { Self { style: Style::new().flex_col().border(1.0).border_color(Color::rgb(226,232,240)).rounded_md().build(), sections } }
    pub fn is_open(&self, idx: usize) -> bool { idx < self.sections.len() && self.sections[idx].open }
}

impl Widget for Accordion {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Accordion" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, Color::WHITE, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 1.0);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { pos, .. } = event {
            let header_h = 36.0; let mut y = ctx.current_rect.min.y;
            for (i, _) in self.sections.iter().enumerate() {
                let hr = Rect::from_xywh(ctx.current_rect.min.x, y, ctx.current_rect.width(), header_h);
                if hr.contains(*pos) { self.sections[i].open = !self.sections[i].open; ctx.state.request_layout(); ctx.state.request_redraw(); return EventResult::HandledAndRedraw; }
                y += header_h; if self.sections[i].open { y += 60.0; }
            }
        }
        EventResult::Ignored
    }
}
