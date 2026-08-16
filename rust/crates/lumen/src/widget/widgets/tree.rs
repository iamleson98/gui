//! Tree view widget.
use crate::core::{Color, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use smol_str::SmolStr;

pub struct TreeNode { pub label: SmolStr, pub children: Vec<TreeNode>, pub expanded: bool, pub disabled: bool }
impl TreeNode {
    pub fn new(label: impl Into<SmolStr>) -> Self { Self { label: label.into(), children: Vec::new(), expanded: false, disabled: false } }
    pub fn with_children(mut self, c: Vec<TreeNode>) -> Self { self.children = c; self }
    pub fn expanded(mut self) -> Self { self.expanded = true; self }
}

pub struct Tree { style: ResolvedStyle, root: TreeNode }
impl Tree { pub fn new(root: TreeNode) -> Self { Self { style: Style::new().flex_col().bg_white().text_sm().build(), root } } }

impl Widget for Tree {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Tree" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
