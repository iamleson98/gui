//! Date picker widget.
use crate::core::Rect;
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};

pub struct Date { pub year: i32, pub month: u8, pub day: u8 }
impl Date {
    pub fn new(y: i32, m: u8, d: u8) -> Self { Self { year: y, month: m, day: d } }
    pub fn days_in_month(y: i32, m: u8) -> u8 { match m { 1|3|5|7|8|10|12=>31, 4|6|9|11=>30, 2=>if Self::is_leap(y) {29} else {28}, _=>30 } }
    pub fn is_leap(y: i32) -> bool { (y%4==0 && y%100!=0) || (y%400==0) }
}

pub struct DatePicker { style: ResolvedStyle, date: Date, open: bool }
impl DatePicker {
    pub fn new(date: Date) -> Self { Self { style: Style::new().px_3().py_2().rounded_md().bg_white().border(1.0).cursor_pointer().text_sm().build(), date, open: false } }
    pub fn date(&self) -> &Date { &self.date }
}

impl Widget for DatePicker {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "DatePicker" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        ctx.painter.stroke_rect(*rect, self.style.border_color, 1.0);
    }
}
