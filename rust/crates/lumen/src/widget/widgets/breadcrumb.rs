//! Breadcrumb navigation widget.
use crate::core::Rect;
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct Crumb { pub label: SmolStr, pub clickable: bool }
impl Crumb { pub fn new(label: impl Into<SmolStr>) -> Self { Self { label: label.into(), clickable: true } } }

pub struct Breadcrumb { style: ResolvedStyle, crumbs: Vec<Crumb> }
impl Breadcrumb {
    pub fn new(crumbs: Vec<Crumb>) -> Self { Self { style: Style::new().flex().gap_1().items_center().text_sm().build(), crumbs } }
    pub fn crumbs(&self) -> &[Crumb] { &self.crumbs }
    pub fn push(&mut self, c: Crumb) { self.crumbs.push(c); }
    pub fn pop(&mut self) { self.crumbs.pop(); }
}

impl Widget for Breadcrumb {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Breadcrumb" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
