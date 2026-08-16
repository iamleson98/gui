mod color;
mod geom;
mod id;
mod rect;
pub use color::Color;
pub use geom::{vec2, Vec2};
pub use id::Id;
pub use rect::Rect;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ScaleFactor(pub f32);
impl ScaleFactor {
    pub const IDENT: Self = Self(1.0);
    pub fn as_f32(self) -> f32 { self.0 }
}
