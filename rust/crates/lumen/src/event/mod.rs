use crate::core::{Id, Rect, Vec2};
use crate::input::{KeyCode, Modifiers, MouseButton};
use crate::style::Cursor;
use std::any::Any;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    PointerMove { pos: Vec2 },
    PointerDown { pos: Vec2, button: MouseButton },
    PointerUp { pos: Vec2, button: MouseButton },
    PointerLeave,
    Scroll { pos: Vec2, delta: Vec2 },
    KeyDown { code: KeyCode, modifiers: Modifiers },
    KeyUp { code: KeyCode, modifiers: Modifiers },
    Char { c: char },
    FocusGained,
    FocusLost,
    Resized { size: Vec2 },
    RequestRedraw { id: Id },
}
impl Event {
    pub fn position(&self) -> Option<Vec2> {
        match self {
            Event::PointerMove { pos }
            | Event::PointerDown { pos, .. }
            | Event::PointerUp { pos, .. }
            | Event::Scroll { pos, .. } => Some(*pos),
            _ => None,
        }
    }
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventResult {
    #[default]
    Ignored,
    Handled,
    HandledAndRedraw,
}
pub struct EventCtx<'a> {
    pub current_id: Id,
    pub current_rect: Rect,
    pub state: &'a mut EventState,
}
#[derive(Default)]
pub struct EventState {
    pub needs_redraw: bool,
    pub needs_layout: bool,
    pub focus_request: Option<Id>,
    pub cursor: Option<Cursor>,
    pub pointer_capture: Option<Id>,
    pub messages: Vec<Box<dyn Any + Send>>,
}
impl EventState {
    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }
    pub fn request_layout(&mut self) {
        self.needs_layout = true;
        self.needs_redraw = true;
    }
    pub fn request_focus(&mut self, id: Id) {
        self.focus_request = Some(id);
    }
    pub fn set_cursor(&mut self, c: Cursor) {
        self.cursor = Some(c);
    }
    pub fn capture_pointer(&mut self, id: Id) {
        self.pointer_capture = Some(id);
    }
    pub fn release_pointer(&mut self) {
        self.pointer_capture = None;
    }
    pub fn emit<T: 'static + Send>(&mut self, msg: T) {
        self.messages.push(Box::new(msg));
    }
}
pub struct Message {
    inner: Box<dyn Any + Send>,
}
impl Message {
    pub fn new<T: 'static + Send>(v: T) -> Self {
        Self { inner: Box::new(v) }
    }
    pub fn type_id(&self) -> std::any::TypeId {
        (*self.inner).type_id()
    }
    pub fn downcast<T: 'static + Clone>(&self) -> Option<T> {
        self.inner.downcast_ref::<T>().cloned()
    }
}
pub struct MessageBus {
    slots: std::collections::HashMap<std::any::TypeId, Vec<Box<dyn Fn(&(dyn Any + Send))>>>,
}
impl Default for MessageBus {
    fn default() -> Self {
        Self {
            slots: Default::default(),
        }
    }
}
impl MessageBus {
    pub fn subscribe<T: 'static + Send + Clone>(&mut self, f: impl Fn(T) + 'static) {
        let tid = std::any::TypeId::of::<T>();
        self.slots
            .entry(tid)
            .or_default()
            .push(Box::new(move |m: &(dyn Any + Send)| {
                if let Some(v) = m.downcast_ref::<T>() {
                    f(v.clone());
                }
            }));
    }
    pub fn dispatch(&self, msg: &(dyn Any + Send)) {
        if let Some(slots) = self.slots.get(&(*msg).type_id()) {
            for s in slots {
                s(msg);
            }
        }
    }
}
