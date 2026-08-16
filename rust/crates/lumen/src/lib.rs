#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
extern crate alloc;
pub mod core;
pub mod event;
pub mod input;
pub mod layout;
pub mod platform;
pub mod render;
pub mod style;
pub mod text;
pub mod widget;
pub mod state;
pub mod animation;
pub mod focus;
pub mod accessibility;
pub mod theme_observer;
pub mod lifecycle;
pub mod error_boundary;
pub mod clipboard;
pub mod prelude {
    pub use crate::core::{Color, Id, Rect, ScaleFactor, Vec2};
    pub use crate::event::{Event, EventCtx, EventResult, Message};
    pub use crate::platform::{App, AppBuilder};
    pub use crate::style::{Style, Tailwind, Tw, Theme, ThemeKind};
    pub use crate::widget::{Element, Ui, Widget};
    pub use crate::widget::widgets::*;
}
