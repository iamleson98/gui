use crate::core::{Color, Rect};
use crate::style::Corners;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex { pub pos: [f32; 2], pub color: [f32; 4], pub uv: [u16; 4], pub z: f32, pub kind: u32 }

pub type Index = u32;

#[derive(Default)]
pub struct Mesh { pub vertices: Vec<Vertex>, pub indices: Vec<Index> }
impl Mesh {
    pub fn new() -> Self { Self::default() }
    pub fn clear(&mut self) { self.vertices.clear(); self.indices.clear(); }
    pub fn empty(&self) -> bool { self.vertices.is_empty() }
    pub fn vertex_count(&self) -> usize { self.vertices.len() }
    pub fn index_count(&self) -> usize { self.indices.len() }
    pub fn add_rect(&mut self, r: Rect, color: Color, z: f32) {
        let col = color.to_linear_premul();
        let i = self.vertices.len() as u32;
        self.vertices.extend([
            Vertex { pos: [r.min.x, r.min.y], color: col, uv: [0;4], z, kind: 0 },
            Vertex { pos: [r.max.x, r.min.y], color: col, uv: [0;4], z, kind: 0 },
            Vertex { pos: [r.max.x, r.max.y], color: col, uv: [0;4], z, kind: 0 },
            Vertex { pos: [r.min.x, r.max.y], color: col, uv: [0;4], z, kind: 0 },
        ]);
        self.indices.extend([i, i+1, i+2, i, i+2, i+3]);
    }
    pub fn add_glyph(&mut self, r: Rect, uv: [u16; 4], color: Color, z: f32) {
        let col = color.to_linear_premul();
        let i = self.vertices.len() as u32;
        self.vertices.extend([
            Vertex { pos: [r.min.x, r.min.y], color: col, uv, z, kind: 1 },
            Vertex { pos: [r.max.x, r.min.y], color: col, uv, z, kind: 1 },
            Vertex { pos: [r.max.x, r.max.y], color: col, uv, z, kind: 1 },
            Vertex { pos: [r.min.x, r.max.y], color: col, uv, z, kind: 1 },
        ]);
        self.indices.extend([i, i+1, i+2, i, i+2, i+3]);
    }
    pub fn add_rounded_rect(&mut self, r: Rect, color: Color, radius: Corners, z: f32) {
        let col = color.to_linear_premul();
        let r_tl = radius.top_left.min(r.width()*0.5).min(r.height()*0.5);
        let r_tr = radius.top_right.min(r.width()*0.5).min(r.height()*0.5);
        let r_br = radius.bottom_right.min(r.width()*0.5).min(r.height()*0.5);
        let r_bl = radius.bottom_left.min(r.width()*0.5).min(r.height()*0.5);
        let center = Rect::from_xywh(r.min.x + r_tl, r.min.y, r.width() - r_tl - r_tr, r.height());
        self.add_rect(center, color, z);
        if r_tl > 0.0 || r_bl > 0.0 { self.add_rect(Rect::from_xywh(r.min.x, r.min.y + r_tl, r_tl, r.height() - r_tl - r_bl), color, z); }
        if r_tr > 0.0 || r_br > 0.0 { self.add_rect(Rect::from_xywh(r.max.x - r_tr, r.min.y + r_tr, r_tr, r.height() - r_tr - r_br), color, z); }
        const STEPS: usize = 8;
        let corners = [(r.min.x+r_tl, r.min.y+r_tl, r_tl, 180.0, 270.0), (r.max.x-r_tr, r.min.y+r_tr, r_tr, 270.0, 360.0), (r.max.x-r_br, r.max.y-r_br, r_br, 0.0, 90.0), (r.min.x+r_bl, r.max.y-r_bl, r_bl, 90.0, 180.0)];
        for (cx, cy, rad, a0, a1) in corners {
            if rad < 0.5 { continue; }
            let base = self.vertices.len() as u32;
            self.vertices.push(Vertex { pos: [cx, cy], color: col, uv: [0;4], z, kind: 0 });
            for i in 0..=STEPS {
                let t = i as f32 / STEPS as f32;
                let a = (a0 + (a1 - a0) * t).to_radians();
                self.vertices.push(Vertex { pos: [cx + rad * a.cos(), cy + rad * a.sin()], color: col, uv: [0;4], z, kind: 0 });
            }
            for i in 0..STEPS as u32 { self.indices.extend([base, base+1+i, base+2+i]); }
        }
    }
}
