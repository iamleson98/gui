//! Pagination widget.
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::Widget;

pub struct Pagination { style: ResolvedStyle, total: usize, current: usize }
impl Pagination {
    pub fn new(total: usize, current: usize) -> Self { Self { style: Style::new().flex().gap_1().build(), total, current: current.min(total.saturating_sub(1)) } }
    pub fn current(&self) -> usize { self.current }
    pub fn total_pages(&self) -> usize { self.total }
}

impl Widget for Pagination {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Pagination" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
