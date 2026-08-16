#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(clippy::type_complexity)]
extern crate alloc;
pub mod accessibility;
pub mod animation;
pub mod clipboard;
pub mod core;
pub mod error_boundary;
pub mod event;
pub mod focus;
pub mod input;
pub mod layout;
pub mod lifecycle;
pub mod platform;
pub mod render;
pub mod state;
pub mod style;
pub mod text;
pub mod theme_observer;
pub mod widget;
pub mod prelude {
    pub use crate::core::{Color, Id, Rect, ScaleFactor, Vec2};
    pub use crate::event::{Event, EventCtx, EventResult, Message};
    pub use crate::platform::{App, AppBuilder};
    pub use crate::style::{Style, Tailwind, Theme, ThemeKind, Tw};
    pub use crate::widget::widgets::*;
    pub use crate::widget::{Element, Ui, Widget};
}
