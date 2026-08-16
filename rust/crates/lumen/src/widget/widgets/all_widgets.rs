use crate::core::{Color, Id, Rect, Vec2};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{Element, PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Button { style: ResolvedStyle, label: SmolStr, hovered: bool, pressed: bool, on_click: Option<Box<dyn Fn(&Id) + Send + Sync>> }
impl Button {
    pub fn new(label: impl Into<SmolStr>) -> Self {
        Self { style: Style::new().px_4().py_2().rounded_md().bg_primary().text_white().font_medium().cursor_pointer().build(), label: label.into(), hovered: false, pressed: false, on_click: None }
    }
    pub fn on_click(mut self, f: impl Fn(&Id) + Send + Sync + 'static) -> Self { self.on_click = Some(Box::new(f)); self }
}
impl Widget for Button {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Button" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let mut c = self.style.background;
        if self.pressed { c = c.lerp(Color::BLACK, 0.15); } else if self.hovered { c = c.lerp(Color::WHITE, 0.10); }
        ctx.painter.fill_rounded_rect(*rect, c, self.style.border_radius);
        let _ = (&self.label, ctx);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerMove { .. } => { if !self.hovered { self.hovered = true; ctx.state.request_redraw(); ctx.state.set_cursor(crate::style::Cursor::Pointer); EventResult::Handled } else { EventResult::Ignored } }
            Event::PointerLeave => { if self.hovered || self.pressed { self.hovered = false; self.pressed = false; ctx.state.request_redraw(); EventResult::Handled } else { EventResult::Ignored } }
            Event::PointerDown { .. } => { self.pressed = true; ctx.state.capture_pointer(ctx.current_id); EventResult::Handled }
            Event::PointerUp { .. } => { if self.pressed { self.pressed = false; if let Some(f) = &self.on_click { f(&ctx.current_id); } ctx.state.release_pointer(); EventResult::HandledAndRedraw } else { EventResult::Ignored } }
            _ => EventResult::Ignored,
        }
    }
}

pub struct Label { style: ResolvedStyle, text: SmolStr }
impl Label {
    pub fn new(text: impl Into<SmolStr>) -> Self { Self { style: Style::new().text_sm().build(), text: text.into() } }
    pub fn with_style(mut self, s: ResolvedStyle) -> Self { self.style = s; self }
    pub fn text(&self) -> &str { &self.text }
}
impl Widget for Label {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Label" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { if self.style.background.a > 0 { ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius); } }
}

pub struct Container { style: ResolvedStyle, children: Vec<Element> }
impl Container { pub fn new(style: ResolvedStyle, children: Vec<Element>) -> Self { Self { style, children } } }
impl Widget for Container {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn children(&self) -> &[Element] { &self.children }
    fn children_mut(&mut self) -> &mut [Element] { &mut self.children }
    fn debug_name(&self) -> &'static str { "Container" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

pub struct Checkbox { style: ResolvedStyle, checked: bool, hovered: bool }
impl Checkbox {
    pub fn new(checked: bool) -> Self { Self { style: Style::new().w(18.0).h(18.0).rounded_md().border(2.0).border_color(Color::rgb(148,163,184)).bg_white().cursor_pointer().build(), checked, hovered: false } }
    pub fn checked(&self) -> bool { self.checked }
}
impl Widget for Checkbox {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Checkbox" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let bg = if self.checked { Color::TW_INDIGO_500 } else if self.hovered { Color::rgb(241,245,249) } else { self.style.background };
        ctx.painter.fill_rounded_rect(*rect, bg, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 2.0);
        if self.checked { ctx.painter.fill_rect(rect.inset(4.0), Color::WHITE); }
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        match event {
            Event::PointerDown { .. } => { ctx.state.capture_pointer(ctx.current_id); EventResult::Handled }
            Event::PointerUp { .. } => { self.checked = !self.checked; ctx.state.release_pointer(); EventResult::HandledAndRedraw }
            _ => EventResult::Ignored,
        }
    }
}

pub struct Slider { style: ResolvedStyle, min: f32, max: f32, value: f32, dragging: bool }
impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self { Self { style: Style::new().h(8.0).w_full().rounded_md().bg(Color::rgb(226,232,240)).cursor_pointer().build(), min, max, value: value.clamp(min, max), dragging: false } }
    pub fn value(&self) -> f32 { self.value }
}
impl Widget for Slider {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Slider" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let t = (self.value - self.min) / (self.max - self.min).max(1e-6);
        ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width() * t, rect.height()), Color::TW_INDIGO_500, self.style.border_radius);
    }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerDown { pos, .. } = event { self.dragging = true; ctx.state.capture_pointer(ctx.current_id); let t = ((pos.x - ctx.current_rect.min.x) / ctx.current_rect.width()).clamp(0.0, 1.0); self.value = self.min + t * (self.max - self.min); EventResult::HandledAndRedraw }
        else if let Event::PointerUp { .. } = event { if self.dragging { self.dragging = false; ctx.state.release_pointer(); EventResult::HandledAndRedraw } else { EventResult::Ignored } }
        else { EventResult::Ignored }
    }
}

pub struct Progress { style: ResolvedStyle, value: f32 }
impl Progress { pub fn new(value: f32) -> Self { Self { style: Style::new().h(8.0).w_full().rounded_md().bg(Color::rgb(226,232,240)).build(), value: value.clamp(0.0, 1.0) } } }
impl Widget for Progress {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Progress" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width() * self.value, rect.height()), Color::TW_INDIGO_500, self.style.border_radius);
    }
}

pub struct Toggle { style: ResolvedStyle, on: bool }
impl Toggle { pub fn new(on: bool) -> Self { Self { style: Style::new().w(44.0).h(24.0).rounded_full().bg(Color::rgb(203,213,225)).cursor_pointer().build(), on } } pub fn is_on(&self) -> bool { self.on } }
impl Widget for Toggle {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Toggle" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        let track = if self.on { Color::TW_EMERALD_500 } else { Color::rgb(203,213,225) };
        ctx.painter.fill_rounded_rect(*rect, track, self.style.border_radius);
        let r = rect.height() * 0.4; let travel = rect.width() - r * 2.0 - 4.0;
        let x = rect.min.x + 2.0 + r + travel * (if self.on { 1.0 } else { 0.0 });
        let y = rect.center().y;
        ctx.painter.fill_rounded_rect(Rect::from_xywh(x-r, y-r, r*2.0, r*2.0), Color::WHITE, crate::style::Corners::all(r));
    }
    fn on_event(&mut self, _ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::PointerUp { .. } = event { self.on = !self.on; EventResult::HandledAndRedraw } else if let Event::PointerDown { .. } = event { EventResult::Handled } else { EventResult::Ignored }
    }
}

pub struct Badge { style: ResolvedStyle, color: Color }
impl Badge { pub fn new(_text: impl Into<SmolStr>, color: Color) -> Self { Self { style: Style::new().px_2().py_1().rounded_full().build(), color } } }
impl Widget for Badge {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Badge" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { ctx.painter.fill_rounded_rect(*rect, self.color, self.style.border_radius); }
}

pub struct Avatar { style: ResolvedStyle, bg: Color }
impl Avatar { pub fn new(initials: impl Into<SmolStr>, size: f32) -> Self { let i: SmolStr = initials.into(); let bg = avatar_color(&i); Self { style: Style::new().w(size).h(size).rounded_full().bg(bg).build(), bg } } }
impl Widget for Avatar {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Avatar" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { ctx.painter.fill_rounded_rect(*rect, self.bg, self.style.border_radius); }
}
fn avatar_color(s: &str) -> Color {
    use std::hash::{Hash, Hasher};
    let palette = [Color::TW_INDIGO_500, Color::TW_EMERALD_500, Color::TW_ROSE_500, Color::TW_AMBER_500];
    let mut h = ahash::AHasher::default(); s.hash(&mut h); palette[(h.finish() as usize) % palette.len()]
}

pub struct Scroll { style: ResolvedStyle, children: Vec<Element>, offset: Vec2 }
impl Scroll { pub fn new(children: Vec<Element>) -> Self { Self { style: Style::new().overflow_hidden().rounded_md().bg(Color::TRANSPARENT).build(), children, offset: Vec2::ZERO } } }
impl Widget for Scroll {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn children(&self) -> &[Element] { &self.children }
    fn children_mut(&mut self) -> &mut [Element] { &mut self.children }
    fn debug_name(&self) -> &'static str { "Scroll" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { if self.style.background.a > 0 { ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius); } ctx.painter.push_clip(*rect); ctx.painter.translate(Vec2::new(-self.offset.x, -self.offset.y)); }
    fn on_event(&mut self, ctx: &mut EventCtx<'_>, event: &Event) -> EventResult {
        if let Event::Scroll { delta, .. } = event { self.offset.y = (self.offset.y + delta.y * 20.0).max(0.0); EventResult::HandledAndRedraw } else { EventResult::Ignored }
    }
}

pub struct Card { style: ResolvedStyle, body: Vec<Element> }
impl Card { pub fn new(body: Vec<Element>) -> Self { Self { style: Style::new().p_4().rounded_lg().bg_white().shadow_md().flex_col().gap_2().build(), body } } }
impl Widget for Card {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn children(&self) -> &[Element] { &self.body }
    fn children_mut(&mut self) -> &mut [Element] { &mut self.body }
    fn debug_name(&self) -> &'static str { "Card" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) { ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius); }
}
