use crate::core::Vec2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect { pub min: Vec2, pub max: Vec2 }
impl Rect {
    pub const ZERO: Self = Self { min: Vec2::ZERO, max: Vec2::ZERO };
    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self { Self { min: Vec2::new(x,y), max: Vec2::new(x+w, y+h) } }
    pub fn width(self) -> f32 { self.max.x - self.min.x }
    pub fn height(self) -> f32 { self.max.y - self.min.y }
    pub fn center(self) -> Vec2 { Vec2::new((self.min.x+self.max.x)*0.5, (self.min.y+self.max.y)*0.5) }
    pub fn contains(self, p: Vec2) -> bool { p.x >= self.min.x && p.x < self.max.x && p.y >= self.min.y && p.y < self.max.y }
    pub fn translate(self, by: Vec2) -> Self { Self { min: self.min+by, max: self.max+by } }
    pub fn inset(self, by: f32) -> Self { Self::from_xywh(self.min.x+by, self.min.y+by, (self.width()-2.0*by).max(0.0), (self.height()-2.0*by).max(0.0)) }
    pub fn intersect(self, o: Self) -> Self { let mn = self.min.max(o.min); let mx = self.max.min(o.max); if mn.x <= mx.x && mn.y <= mx.y { Self { min: mn, max: mx } } else { Self::ZERO } }
}
