//! Data table widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct TableColumn { pub id: SmolStr, pub label: SmolStr, pub width: f32 }
impl TableColumn { pub fn new(id: impl Into<SmolStr>, label: impl Into<SmolStr>, width: f32) -> Self { Self { id: id.into(), label: label.into(), width } } }
pub struct TableRow { pub cells: Vec<SmolStr> }
impl TableRow { pub fn new(cells: Vec<impl Into<SmolStr>>) -> Self { Self { cells: cells.into_iter().map(Into::into).collect() } } pub fn cell(&self, col: usize) -> &str { self.cells.get(col).map(|s| s.as_str()).unwrap_or("") } }

pub struct Table { style: ResolvedStyle, columns: Vec<TableColumn>, rows: Vec<TableRow>, selected: Vec<usize> }
impl Table {
    pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
        Self { style: Style::new().flex_col().bg_white().rounded_md().border(1.0).border_color(Color::rgb(226,232,240)).text_sm().build(), columns, rows, selected: Vec::new() }
    }
    pub fn row_count(&self) -> usize { self.rows.len() }
    pub fn selected(&self) -> &[usize] { &self.selected }
}

impl Widget for Table {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Table" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        ctx.painter.fill_rounded_rect(*rect, self.style.background, self.style.border_radius);
        let header = Rect::from_xywh(rect.min.x, rect.min.y, rect.width(), 36.0);
        ctx.painter.fill_rect(header, Color::rgb(248,250,252));
    }
}
