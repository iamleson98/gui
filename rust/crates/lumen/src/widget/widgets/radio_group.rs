//! Radio group widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct RadioGroup {
    style: ResolvedStyle,
    options: Vec<SmolStr>,
    selected: Option<usize>,
}

impl RadioGroup {
    pub fn new(options: Vec<impl Into<SmolStr>>) -> Self {
        Self { style: Style::new().flex_col().gap_2().build(), options: options.into_iter().map(Into::into).collect(), selected: None }
    }
    pub fn with_selected(mut self, idx: usize) -> Self { if idx < self.options.len() { self.selected = Some(idx); } self }
    pub fn selected(&self) -> Option<usize> { self.selected }
}

impl Widget for RadioGroup {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "RadioGroup" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let n = self.options.len() as f32;
        if n == 0.0 { return; }
        let row_h = rect.height() / n;
        for (i, _) in self.options.iter().enumerate() {
            let y = rect.min.y + row_h * i as f32;
            let sel = self.selected == Some(i);
            let cx = rect.min.x + 8.0; let cy = y + row_h * 0.5; let r = 8.0;
            let outer = if sel { Color::TW_INDIGO_500 } else { Color::rgb(203,213,225) };
            ctx.painter.fill_rounded_rect(Rect::from_xywh(cx - r, cy - r, r * 2.0, r * 2.0), Color::WHITE, crate::style::Corners::all(r));
            ctx.painter.stroke_rect(Rect::from_xywh(cx - r, cy - r, r * 2.0, r * 2.0), outer, 2.0);
            if sel { ctx.painter.fill_rounded_rect(Rect::from_xywh(cx - 4.0, cy - 4.0, 8.0, 8.0), Color::TW_INDIGO_500, crate::style::Corners::all(4.0)); }
        }
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { pos, .. } = event {
            let n = self.options.len(); if n == 0 { return EventResult::Ignored; }
            let row_h = ctx.current_rect.height() / n as f32;
            let rel_y = pos.y - ctx.current_rect.min.y;
            if rel_y >= 0.0 && rel_y <= ctx.current_rect.height() {
                let idx = (rel_y / row_h) as usize;
                if idx < n { self.selected = Some(idx); ctx.state.request_redraw(); return EventResult::HandledAndRedraw; }
            }
        }
        EventResult::Ignored
    }
}
