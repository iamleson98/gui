use crate::style::ResolvedStyle;
use crate::widget::{Element, Widget};

pub struct Container {
    style: ResolvedStyle,
    children: Vec<Element>,
}
impl Container {
    pub fn new(style: ResolvedStyle, children: Vec<Element>) -> Self {
        Self { style, children }
    }
}
impl Widget for Container {
    fn style(&self) -> &ResolvedStyle {
        &self.style
    }
    fn children(&self) -> &[Element] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut [Element] {
        &mut self.children
    }
    fn debug_name(&self) -> &'static str {
        "Container"
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
