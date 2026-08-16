pub mod widgets;
use crate::core::{Id, Rect};
use crate::event::{Event, EventCtx, EventResult};
use crate::render::Painter;
use crate::style::ResolvedStyle;
use crate::text::TextEngine;

pub struct PaintCtx<'a> {
    pub painter: &'a mut Painter,
    pub theme: &'a crate::style::Theme,
    pub layout: &'a crate::layout::LayoutRect,
    pub scale: crate::core::ScaleFactor,
    pub text: &'a mut TextEngine,
}

pub trait Widget: Send + Sync {
    fn style(&self) -> &ResolvedStyle;
    fn paint(&self, _ctx: &mut PaintCtx<'_>, _rect: &Rect) {}
    fn on_event(&mut self, _ctx: &mut EventCtx<'_>, _event: &Event) -> EventResult {
        EventResult::Ignored
    }
    fn children(&self) -> &[Element] {
        &[]
    }
    fn children_mut(&mut self) -> &mut [Element] {
        &mut []
    }
    fn debug_name(&self) -> &'static str {
        "Widget"
    }
    /// If this widget renders text, return `Some((text, font_size))` so the
    /// layout engine can measure it and use the result as the widget's
    /// intrinsic content size. Default is `None`.
    fn text_measure(&self) -> Option<(&str, f32)> {
        None
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct Element {
    pub inner: Box<dyn Widget>,
    pub id: Id,
}
impl Element {
    pub fn new<W: Widget + 'static>(id: Id, w: W) -> Self {
        Self {
            inner: Box::new(w),
            id,
        }
    }
}

pub struct Ui {
    id: Id,
    children: Vec<Element>,
    next_index: usize,
}
impl Ui {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            children: Vec::new(),
            next_index: 0,
        }
    }
    pub fn push<W: Widget + 'static>(&mut self, widget: W) -> &mut W {
        let id = self.id.derive_index(self.next_index);
        self.next_index += 1;
        self.children.push(Element::new(id, widget));
        let last = self.children.last_mut().unwrap();
        last.inner
            .as_any_mut_safe()
            .downcast_mut::<W>()
            .expect("type mismatch")
    }
    pub fn into_children(self) -> Vec<Element> {
        self.children
    }
}

pub trait AsAnyMut {
    fn as_any_mut_safe(&mut self) -> &mut dyn std::any::Any;
}
impl<W: Widget + 'static> AsAnyMut for W {
    fn as_any_mut_safe(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
impl dyn Widget {
    pub fn as_any_mut_safe(&mut self) -> &mut dyn std::any::Any {
        Widget::as_any_mut(self)
    }
}
