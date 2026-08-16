#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 200, 0);
    pub const BLUE: Self = Self::rgb(0, 90, 220);
    pub const YELLOW: Self = Self::rgb(255, 220, 0);
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    pub const TW_INDIGO_500: Self = Self::rgb(99, 102, 241);
    pub const TW_EMERALD_500: Self = Self::rgb(16, 185, 129);
    pub const TW_ROSE_500: Self = Self::rgb(244, 63, 94);
    pub const TW_AMBER_500: Self = Self::rgb(245, 158, 11);
    pub const TW_SLATE_900: Self = Self::rgb(15, 23, 42);
    pub const TW_SLATE_100: Self = Self::rgb(241, 245, 249);
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn with_alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
    pub fn alpha_f32(self) -> f32 {
        self.a as f32 / 255.0
    }
    pub fn to_linear_premul(self) -> [f32; 4] {
        let a = self.alpha_f32();
        let s2l = |c: u8| {
            let f = c as f32 / 255.0;
            if f <= 0.04045 {
                f / 12.92
            } else {
                ((f + 0.055) / 1.055).powf(2.4)
            }
        };
        [s2l(self.r) * a, s2l(self.g) * a, s2l(self.b) * a, a]
    }
    pub fn lerp(self, o: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let l = |a: u8, b: u8| -> u8 { (a as f32 + (b as f32 - a as f32) * t).round() as u8 };
        Self::rgba(
            l(self.r, o.r),
            l(self.g, o.g),
            l(self.b, o.b),
            l(self.a, o.a),
        )
    }
}

impl Color {
    pub fn from_hex(hex: &str) -> Option<Self> {
        let b = hex.as_bytes();
        if b.len() < 7 || b[0] != b'#' {
            return None;
        }
        let hp = |i: usize| -> Option<u8> {
            let d = |c: u8| match c {
                b'0'..=b'9' => Some(c - b'0'),
                b'a'..=b'f' => Some(c - b'a' + 10),
                b'A'..=b'F' => Some(c - b'A' + 10),
                _ => None,
            };
            Some((d(b[i])? << 4) | d(b[i + 1])?)
        };
        match b.len() {
            7 => Some(Self::rgba(hp(1)?, hp(3)?, hp(5)?, 255)),
            9 => Some(Self::rgba(hp(1)?, hp(3)?, hp(5)?, hp(7)?)),
            _ => None,
        }
    }
}
