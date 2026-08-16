pub mod mesh;
pub mod svg;
pub mod wgpu_backend;
pub use mesh::{Mesh, Vertex};
pub use svg::{fill_svg_path, SvgPath};
pub use wgpu_backend::{SurfaceConfig, WgpuRenderer};
pub type Index = u32;

use crate::core::{Color, Rect, Vec2};
use crate::style::Corners;
use smallvec::SmallVec;

pub struct Painter {
    mesh: Mesh,
    clip_stack: SmallVec<[crate::core::Rect; 8]>,
    z: f32,
    offset: Vec2,
}
impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}
impl Painter {
    pub fn new() -> Self {
        Self {
            mesh: Mesh::new(),
            clip_stack: SmallVec::new(),
            z: 0.0,
            offset: Vec2::ZERO,
        }
    }
    pub fn clear(&mut self) {
        self.mesh.clear();
        self.clip_stack.clear();
        self.z = 0.0;
        self.offset = Vec2::ZERO;
    }
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    pub fn push_clip(&mut self, rect: crate::core::Rect) {
        let r = rect.translate(self.offset);
        let new_clip = match self.clip_stack.last() {
            Some(p) => p.intersect(r),
            None => r,
        };
        self.clip_stack.push(new_clip);
    }
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }
    pub fn current_clip(&self) -> Option<crate::core::Rect> {
        self.clip_stack.last().copied()
    }
    pub fn translate(&mut self, offset: Vec2) {
        self.offset = self.offset + offset;
    }
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let r = rect.translate(self.offset);
        let clip = self.current_clip().unwrap_or(Rect::from_min_size(
            Vec2::ZERO,
            Vec2::new(f32::INFINITY, f32::INFINITY),
        ));
        let r = r.intersect(clip);
        if r.width() <= 0.0 || r.height() <= 0.0 {
            return;
        }
        self.mesh.add_rect(r, color, self.z);
        self.z += 1.0 / 65536.0;
    }
    pub fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: Corners) {
        let r = rect.translate(self.offset);
        self.mesh.add_rounded_rect(r, color, radius, self.z);
        self.z += 1.0 / 65536.0;
    }
    pub fn stroke_rect(&mut self, rect: Rect, color: Color, width: f32) {
        let r = rect.translate(self.offset);
        self.fill_rect(Rect::from_xywh(r.min.x, r.min.y, r.width(), width), color);
        self.fill_rect(
            Rect::from_xywh(r.min.x, r.max.y - width, r.width(), width),
            color,
        );
        self.fill_rect(
            Rect::from_xywh(r.min.x, r.min.y + width * 0.5, width, r.height() - width),
            color,
        );
        self.fill_rect(
            Rect::from_xywh(
                r.max.x - width,
                r.min.y + width * 0.5,
                width,
                r.height() - width,
            ),
            color,
        );
    }
    pub fn push_glyph(&mut self, pos: Rect, uv: [u16; 4], color: Color) {
        let r = pos.translate(self.offset);
        self.mesh.add_glyph(r, uv, color, self.z);
        self.z += 1.0 / 65536.0;
    }
    /// Fill an SVG path inside `dst_rect`. The path is interpreted in the
    /// coordinate space defined by `src_rect` (typically a 24×24 viewBox).
    pub fn fill_svg(&mut self, d: &str, dst_rect: Rect, src_rect: Rect, color: Color) {
        let r = dst_rect.translate(self.offset);
        svg::fill_svg_path(&mut self.mesh, d, r, src_rect, color, self.z);
        self.z += 1.0 / 65536.0;
    }
}
