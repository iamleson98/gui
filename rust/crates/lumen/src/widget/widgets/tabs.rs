//! Tab view widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Tabs {
    style: ResolvedStyle,
    tabs: Vec<(SmolStr, Element)>,
    active: usize,
}

impl Tabs {
    pub fn new(tabs: Vec<(impl Into<SmolStr>, Element)>, active: usize) -> Self {
        let count = tabs.len();
        Self { style: Style::new().flex_col().build(), tabs: tabs.into_iter().map(|(l,e)| (l.into(), e)).collect(), active: active.min(count.saturating_sub(1)) }
    }
    pub fn active(&self) -> usize { self.active }
}

impl Widget for Tabs {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Tabs" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let bar = Rect::from_xywh(rect.min.x, rect.min.y, rect.width(), 36.0);
        ctx.painter.fill_rect(bar, Color::rgb(241,245,249));
        let n = self.tabs.len(); if n == 0 { return; }
        let tw = rect.width() / n as f32;
        for (i, _) in self.tabs.iter().enumerate() {
            let tr = Rect::from_xywh(rect.min.x + i as f32 * tw, rect.min.y, tw, 36.0);
            if i == self.active { ctx.painter.fill_rect(tr, Color::WHITE); ctx.painter.fill_rect(Rect::from_xywh(tr.min.x, tr.max.y - 2.0, tr.width(), 2.0), Color::TW_INDIGO_500); }
        }
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { pos, .. } = event {
            if pos.y < ctx.current_rect.min.y + 36.0 {
                let n = self.tabs.len(); if n == 0 { return EventResult::Ignored; }
                let tw = ctx.current_rect.width() / n as f32;
                let idx = ((pos.x - ctx.current_rect.min.x) / tw) as usize;
                if idx < n && idx != self.active { self.active = idx; ctx.state.request_redraw(); return EventResult::HandledAndRedraw; }
            }
        }
        EventResult::Ignored
    }
}
