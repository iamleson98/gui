use crate::core::{Color, Rect};
use crate::style::Corners;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub uv: [u16; 4],
    pub z: f32,
    pub kind: u32,
}

pub type Index = u32;

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}
impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
    pub fn empty(&self) -> bool {
        self.vertices.is_empty()
    }
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
    pub fn add_rect(&mut self, r: Rect, color: Color, z: f32) {
        let col = color.to_linear_premul();
        let i = self.vertices.len() as u32;
        self.vertices.extend([
            Vertex {
                pos: [r.min.x, r.min.y],
                color: col,
                uv: [0; 4],
                z,
                kind: 0,
            },
            Vertex {
                pos: [r.max.x, r.min.y],
                color: col,
                uv: [0; 4],
                z,
                kind: 0,
            },
            Vertex {
                pos: [r.max.x, r.max.y],
                color: col,
                uv: [0; 4],
                z,
                kind: 0,
            },
            Vertex {
                pos: [r.min.x, r.max.y],
                color: col,
                uv: [0; 4],
                z,
                kind: 0,
            },
        ]);
        self.indices.extend([i, i + 1, i + 2, i, i + 2, i + 3]);
    }

    /// Add a glyph quad. `uv` is `[atlas_x, atlas_y, glyph_w, glyph_h]` in
    /// atlas pixels. Each of the 4 vertices gets its own UV so the quad
    /// samples the correct sub-rectangle of the glyph atlas.
    pub fn add_glyph(&mut self, r: Rect, uv: [u16; 4], color: Color, z: f32) {
        let col = color.to_linear_premul();
        let i = self.vertices.len() as u32;
        let (ux, uy, uw, uh) = (uv[0], uv[1], uv[2], uv[3]);
        self.vertices.extend([
            Vertex {
                // top-left → atlas (ux, uy)
                pos: [r.min.x, r.min.y],
                color: col,
                uv: [ux, uy, 0, 0],
                z,
                kind: 1,
            },
            Vertex {
                // top-right → atlas (ux + uw, uy)
                pos: [r.max.x, r.min.y],
                color: col,
                uv: [ux + uw, uy, 0, 0],
                z,
                kind: 1,
            },
            Vertex {
                // bottom-right → atlas (ux + uw, uy + uh)
                pos: [r.max.x, r.max.y],
                color: col,
                uv: [ux + uw, uy + uh, 0, 0],
                z,
                kind: 1,
            },
            Vertex {
                // bottom-left → atlas (ux, uy + uh)
                pos: [r.min.x, r.max.y],
                color: col,
                uv: [ux, uy + uh, 0, 0],
                z,
                kind: 1,
            },
        ]);
        self.indices.extend([i, i + 1, i + 2, i, i + 2, i + 3]);
    }

    pub fn add_rounded_rect(&mut self, r: Rect, color: Color, radius: Corners, z: f32) {
        let col = color.to_linear_premul();
        let r_tl = radius.top_left.min(r.width() * 0.5).min(r.height() * 0.5);
        let r_tr = radius.top_right.min(r.width() * 0.5).min(r.height() * 0.5);
        let r_br = radius
            .bottom_right
            .min(r.width() * 0.5)
            .min(r.height() * 0.5);
        let r_bl = radius
            .bottom_left
            .min(r.width() * 0.5)
            .min(r.height() * 0.5);
        let center = Rect::from_xywh(r.min.x + r_tl, r.min.y, r.width() - r_tl - r_tr, r.height());
        self.add_rect(center, color, z);
        if r_tl > 0.0 || r_bl > 0.0 {
            self.add_rect(
                Rect::from_xywh(r.min.x, r.min.y + r_tl, r_tl, r.height() - r_tl - r_bl),
                color,
                z,
            );
        }
        if r_tr > 0.0 || r_br > 0.0 {
            self.add_rect(
                Rect::from_xywh(
                    r.max.x - r_tr,
                    r.min.y + r_tr,
                    r_tr,
                    r.height() - r_tr - r_br,
                ),
                color,
                z,
            );
        }
        const STEPS: usize = 8;
        let corners = [
            (r.min.x + r_tl, r.min.y + r_tl, r_tl, 180.0, 270.0),
            (r.max.x - r_tr, r.min.y + r_tr, r_tr, 270.0, 360.0),
            (r.max.x - r_br, r.max.y - r_br, r_br, 0.0, 90.0),
            (r.min.x + r_bl, r.max.y - r_bl, r_bl, 90.0, 180.0),
        ];
        for (cx, cy, rad, a0, a1) in corners {
            if rad < 0.5 {
                continue;
            }
            let base = self.vertices.len() as u32;
            self.vertices.push(Vertex {
                pos: [cx, cy],
                color: col,
                uv: [0; 4],
                z,
                kind: 0,
            });
            for i in 0..=STEPS {
                let t = i as f32 / STEPS as f32;
                let a = (a0 + (a1 - a0) * t).to_radians();
                self.vertices.push(Vertex {
                    pos: [cx + rad * a.cos(), cy + rad * a.sin()],
                    color: col,
                    uv: [0; 4],
                    z,
                    kind: 0,
                });
            }
            for i in 0..STEPS as u32 {
                self.indices.extend([base, base + 1 + i, base + 2 + i]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each of the 4 glyph vertices must have a distinct UV so the quad
    /// samples the full glyph sub-rect rather than just its top-left pixel.
    /// This is the bug that caused text to render as a single dot.
    #[test]
    fn glyph_quad_has_distinct_per_vertex_uvs() {
        let mut mesh = Mesh::new();
        let rect = Rect::from_xywh(0.0, 0.0, 100.0, 100.0);
        // uv = [atlas_x=10, atlas_y=20, glyph_w=30, glyph_h=40]
        mesh.add_glyph(rect, [10, 20, 30, 40], Color::WHITE, 0.0);
        assert_eq!(mesh.vertices.len(), 4);
        let v0 = mesh.vertices[0].uv; // top-left
        let v1 = mesh.vertices[1].uv; // top-right
        let v2 = mesh.vertices[2].uv; // bottom-right
        let v3 = mesh.vertices[3].uv; // bottom-left
        // top-left = (10, 20)
        assert_eq!(v0[0], 10);
        assert_eq!(v0[1], 20);
        // top-right = (10+30, 20) = (40, 20)
        assert_eq!(v1[0], 40);
        assert_eq!(v1[1], 20);
        // bottom-right = (40, 20+40) = (40, 60)
        assert_eq!(v2[0], 40);
        assert_eq!(v2[1], 60);
        // bottom-left = (10, 60)
        assert_eq!(v3[0], 10);
        assert_eq!(v3[1], 60);
    }

    #[test]
    fn add_rounded_rect_does_not_panic() {
        let mut mesh = Mesh::new();
        let r = Rect::from_xywh(0.0, 0.0, 100.0, 50.0);
        mesh.add_rounded_rect(r, Color::WHITE, Corners::all(8.0), 0.0);
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.indices.is_empty());
    }

    #[test]
    fn add_rect_creates_two_triangles() {
        let mut mesh = Mesh::new();
        mesh.add_rect(Rect::from_xywh(0.0, 0.0, 10.0, 10.0), Color::WHITE, 0.0);
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
    }
}
