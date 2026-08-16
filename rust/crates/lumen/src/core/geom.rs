#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 { pub x: f32, pub y: f32 }
impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn min(self, o: Self) -> Self { Self::new(self.x.min(o.x), self.y.min(o.y)) }
    pub fn max(self, o: Self) -> Self { Self::new(self.x.max(o.x), self.y.max(o.y)) }
}
impl std::ops::Add for Vec2 { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.x+r.x, self.y+r.y) } }
impl std::ops::Sub for Vec2 { type Output = Self; fn sub(self, r: Self) -> Self { Self::new(self.x-r.x, self.y-r.y) } }
impl std::ops::Mul<f32> for Vec2 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s, self.y*s) } }
pub const fn vec2(x: f32, y: f32) -> Vec2 { Vec2::new(x, y) }

impl Vec2 {
    pub fn dot(self, o: Self) -> f32 { self.x * o.x + self.y * o.y }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
}
