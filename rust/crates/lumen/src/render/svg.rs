//! SVG path rendering — parses a subset of SVG path data and tessellates it
//! into the existing mesh.
//!
//! Supported commands (absolute and relative):
//!   M / m  moveto
//!   L / l  lineto
//!   H / h  horizontal lineto
//!   V / v  vertical lineto
//!   C / c  cubic bezier (cubicTo)
//!   Q / q  quadratic bezier
//!   A / a  elliptical arc (approximated by sampling)
//!   Z / z  close path
//!
//! The path is rasterised as a triangle fan from the centroid of each
//! sub-path's vertices — simple, fast, good enough for icon-sized glyphs
//! (24×24 viewBox). Stroking is not implemented; only filled paths.

use crate::core::{Color, Rect, Vec2};
use crate::render::Mesh;
use crate::style::Corners;

/// A parsed SVG path, ready to tessellate.
#[derive(Debug, Clone, Default)]
pub struct SvgPath {
    /// Each sub-path is a closed polygon (list of points).
    pub sub_paths: Vec<Vec<Vec2>>,
}

impl SvgPath {
    /// Parse SVG path data. Accepts the standard `d="..."` string format.
    pub fn parse(d: &str) -> Self {
        let mut p = Parser::new(d);
        let mut path = SvgPath {
            sub_paths: Vec::new(),
        };
        let mut cur = Vec2::ZERO;
        let mut start = Vec2::ZERO;
        let mut current: Vec<Vec2> = Vec::new();
        let mut first_cmd = true;

        while let Some((cmd, abs)) = p.next_cmd() {
            match cmd {
                'M' | 'm' => {
                    if !current.is_empty() {
                        path.sub_paths.push(std::mem::take(&mut current));
                    }
                    let (x, y) = p.number_pair();
                    cur = if abs || first_cmd {
                        Vec2::new(x, y)
                    } else {
                        cur + Vec2::new(x, y)
                    };
                    start = cur;
                    current.push(cur);
                    // Implicit linetos after moveto
                    while let Some(_) = p.peek_number() {
                        let (x, y) = p.number_pair();
                        cur = if abs {
                            Vec2::new(x, y)
                        } else {
                            cur + Vec2::new(x, y)
                        };
                        current.push(cur);
                    }
                }
                'L' | 'l' => {
                    let (x, y) = p.number_pair();
                    cur = if abs {
                        Vec2::new(x, y)
                    } else {
                        cur + Vec2::new(x, y)
                    };
                    current.push(cur);
                    while let Some(_) = p.peek_number() {
                        let (x, y) = p.number_pair();
                        cur = if abs {
                            Vec2::new(x, y)
                        } else {
                            cur + Vec2::new(x, y)
                        };
                        current.push(cur);
                    }
                }
                'H' | 'h' => {
                    while let Some(x) = p.number() {
                        cur = if abs {
                            Vec2::new(x, cur.y)
                        } else {
                            Vec2::new(cur.x + x, cur.y)
                        };
                        current.push(cur);
                    }
                }
                'V' | 'v' => {
                    while let Some(y) = p.number() {
                        cur = if abs {
                            Vec2::new(cur.x, y)
                        } else {
                            Vec2::new(cur.x, cur.y + y)
                        };
                        current.push(cur);
                    }
                }
                'C' | 'c' => loop {
                    let (x1, y1) = p.number_pair_or_break();
                    let (x2, y2) = p.number_pair_or_break();
                    let (x, y) = p.number_pair_or_break();
                    if x.is_nan() {
                        break;
                    }
                    let c1 = if abs {
                        Vec2::new(x1, y1)
                    } else {
                        cur + Vec2::new(x1, y1)
                    };
                    let c2 = if abs {
                        Vec2::new(x2, y2)
                    } else {
                        cur + Vec2::new(x2, y2)
                    };
                    let end = if abs {
                        Vec2::new(x, y)
                    } else {
                        cur + Vec2::new(x, y)
                    };
                    sample_cubic(&mut current, cur, c1, c2, end);
                    cur = end;
                },
                'Q' | 'q' => loop {
                    let (x1, y1) = p.number_pair_or_break();
                    let (x, y) = p.number_pair_or_break();
                    if x.is_nan() {
                        break;
                    }
                    let c1 = if abs {
                        Vec2::new(x1, y1)
                    } else {
                        cur + Vec2::new(x1, y1)
                    };
                    let end = if abs {
                        Vec2::new(x, y)
                    } else {
                        cur + Vec2::new(x, y)
                    };
                    sample_quadratic(&mut current, cur, c1, end);
                    cur = end;
                },
                'A' | 'a' => loop {
                    let (rx, ry) = p.number_pair_or_break();
                    if rx.is_nan() {
                        break;
                    }
                    let x_rot = p.number_or_break().unwrap_or(0.0);
                    let large = p.flag_or_break().unwrap_or(0.0) as i32;
                    let sweep = p.flag_or_break().unwrap_or(0.0) as i32;
                    let (x, y) = p.number_pair_or_break();
                    if x.is_nan() {
                        break;
                    }
                    let end = if abs {
                        Vec2::new(x, y)
                    } else {
                        cur + Vec2::new(x, y)
                    };
                    sample_arc(
                        &mut current,
                        cur,
                        rx,
                        ry,
                        x_rot,
                        large != 0,
                        sweep != 0,
                        end,
                    );
                    cur = end;
                },
                'Z' | 'z' => {
                    if !current.is_empty() {
                        current.push(start);
                        path.sub_paths.push(std::mem::take(&mut current));
                    }
                    cur = start;
                }
                _ => {}
            }
            first_cmd = false;
        }
        if !current.is_empty() {
            path.sub_paths.push(current);
        }
        path
    }

    /// Tessellate the path into the mesh as filled triangles (triangle fan
    /// from each sub-path's centroid). The path is mapped from its source
    /// viewBox (`src_rect`) into `dst_rect`.
    pub fn tessellate(
        &self,
        mesh: &mut Mesh,
        dst_rect: Rect,
        src_rect: Rect,
        color: Color,
        z: f32,
    ) {
        let col = color.to_linear_premul();
        let sx = dst_rect.width() / src_rect.width().max(1e-6);
        let sy = dst_rect.height() / src_rect.height().max(1e-6);
        let ox = dst_rect.min.x - src_rect.min.x * sx;
        let oy = dst_rect.min.y - src_rect.min.y * sy;
        let transform = |p: Vec2| Vec2::new(p.x * sx + ox, p.y * sy + oy);

        for sub in &self.sub_paths {
            if sub.len() < 3 {
                continue;
            }
            // Centroid
            let mut cx = 0.0f32;
            let mut cy = 0.0f32;
            for p in sub {
                cx += p.x;
                cy += p.y;
            }
            cx /= sub.len() as f32;
            cy /= sub.len() as f32;
            let center = transform(Vec2::new(cx, cy));
            let base = mesh.vertices.len() as u32;
            mesh.vertices.push(crate::render::Vertex {
                pos: [center.x, center.y],
                color: col,
                uv: [0; 4],
                z,
                kind: 0,
            });
            for p in sub {
                let tp = transform(*p);
                mesh.vertices.push(crate::render::Vertex {
                    pos: [tp.x, tp.y],
                    color: col,
                    uv: [0; 4],
                    z,
                    kind: 0,
                });
            }
            let n = sub.len() as u32;
            for i in 0..n {
                let a = base + 1 + i;
                let b = base + 1 + ((i + 1) % n);
                mesh.indices.extend_from_slice(&[base, a, b]);
            }
        }
    }
}

fn sample_cubic(out: &mut Vec<Vec2>, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) {
    const N: usize = 16;
    for i in 1..=N {
        let t = i as f32 / N as f32;
        let u = 1.0 - t;
        let p =
            p0 * (u * u * u) + p1 * (3.0 * u * u * t) + p2 * (3.0 * u * t * t) + p3 * (t * t * t);
        out.push(p);
    }
}

fn sample_quadratic(out: &mut Vec<Vec2>, p0: Vec2, p1: Vec2, p2: Vec2) {
    const N: usize = 12;
    for i in 1..=N {
        let t = i as f32 / N as f32;
        let u = 1.0 - t;
        let p = p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t);
        out.push(p);
    }
}

fn sample_arc(
    out: &mut Vec<Vec2>,
    start: Vec2,
    rx: f32,
    ry: f32,
    x_rot_deg: f32,
    large: bool,
    sweep: bool,
    end: Vec2,
) {
    // Convert endpoint to center parameterization (SVG spec appendix F)
    let phi = x_rot_deg.to_radians();
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();
    let dx = (start.x - end.x) / 2.0;
    let dy = (start.y - end.y) / 2.0;
    let x1p = cos_phi * dx + sin_phi * dy;
    let y1p = -sin_phi * dx + cos_phi * dy;
    let rx = rx.abs().max(1e-6);
    let ry = ry.abs().max(1e-6);
    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    let s = if lambda > 1.0 { lambda.sqrt() } else { 1.0 };
    let rx = rx * s;
    let ry = ry * s;
    let num = rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p;
    let den = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    let factor = if num < 0.0 || den < 0.0 {
        0.0
    } else {
        (num / den.max(1e-12)).sqrt()
    };
    let sign = if large == sweep { -1.0 } else { 1.0 };
    let cxp = sign * factor * (rx * y1p / ry);
    let cyp = sign * factor * (-ry * x1p / rx);
    let cx = cos_phi * cxp - sin_phi * cyp + (start.x + end.x) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (start.y + end.y) / 2.0;
    let angle = |px: f32, py: f32| {
        let t = ((px - cx) / rx).acos().copysign((py - cy) / ry);
        t
    };
    let mut theta1 = angle(start.x - cx, start.y - cy);
    let mut dtheta = angle(end.x - cx, end.y - cy) - theta1;
    if sweep && dtheta < 0.0 {
        dtheta += 2.0 * std::f32::consts::PI;
    }
    if !sweep && dtheta > 0.0 {
        dtheta -= 2.0 * std::f32::consts::PI;
    }
    let _ = &mut theta1;
    const N: usize = 24;
    for i in 1..=N {
        let t = theta1 + dtheta * (i as f32 / N as f32);
        let px = cx + rx * (cos_phi * t.cos() - sin_phi * t.sin());
        let py = cy + ry * (sin_phi * t.cos() + cos_phi * t.sin());
        out.push(Vec2::new(px, py));
    }
}

struct Parser<'a> {
    chars: std::str::Chars<'a>,
    peek: Option<char>,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars(),
            peek: None,
        }
    }

    fn next_cmd(&mut self) -> Option<(char, bool)> {
        // Skip whitespace and commas
        loop {
            let c = if let Some(c) = self.peek.take() {
                c
            } else {
                match self.chars.next() {
                    Some(c) => c,
                    None => return None,
                }
            };
            if c.is_whitespace() || c == ',' {
                continue;
            }
            if c.is_ascii_alphabetic() {
                return Some((c.to_ascii_uppercase(), c.is_ascii_uppercase()));
            }
            // Number started — push back as a peek so number() can read it.
            self.peek = Some(c);
            // Implicit repeated command — caller will handle by reading numbers
            // but since we can't return the original cmd, return None.
            return None;
        }
    }

    fn peek_number(&mut self) -> Option<()> {
        // Skip whitespace
        loop {
            let c = if let Some(c) = self.peek.take() {
                c
            } else {
                match self.chars.next() {
                    Some(c) => c,
                    None => return None,
                }
            };
            if c.is_whitespace() || c == ',' {
                continue;
            }
            // First non-ws char must be a digit, sign, or dot
            if c.is_ascii_digit() || c == '-' || c == '+' || c == '.' {
                self.peek = Some(c);
                return Some(());
            }
            // It's a command — push back, no more numbers
            self.peek = Some(c);
            return None;
        }
    }

    fn number(&mut self) -> Option<f32> {
        // Skip whitespace
        let mut s = String::new();
        let mut started = false;
        loop {
            let c = if let Some(c) = self.peek.take() {
                c
            } else {
                match self.chars.next() {
                    Some(c) => c,
                    None => break,
                }
            };
            if !started && (c.is_whitespace() || c == ',') {
                continue;
            }
            if c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E' {
                s.push(c);
                started = true;
            } else {
                self.peek = Some(c);
                break;
            }
        }
        if s.is_empty() {
            None
        } else {
            s.parse().ok()
        }
    }

    fn number_or_break(&mut self) -> Option<f32> {
        // Like number() but returns None if no number is available without
        // consuming a non-numeric char
        if self.peek_number().is_none() {
            return None;
        }
        self.number()
    }

    fn number_pair(&mut self) -> (f32, f32) {
        let x = self.number().unwrap_or(0.0);
        let y = self.number().unwrap_or(0.0);
        (x, y)
    }

    fn number_pair_or_break(&mut self) -> (f32, f32) {
        if self.peek_number().is_none() {
            return (f32::NAN, f32::NAN);
        }
        let x = self.number().unwrap_or(0.0);
        let y = self.number().unwrap_or(0.0);
        (x, y)
    }

    fn flag_or_break(&mut self) -> Option<f32> {
        if self.peek_number().is_none() {
            return None;
        }
        // Flags are 0 or 1, no whitespace required between them, but we'll be loose
        self.number()
    }
}

/// Convenience: add a single filled SVG path to the mesh.
pub fn fill_svg_path(
    mesh: &mut Mesh,
    d: &str,
    dst_rect: Rect,
    src_rect: Rect,
    color: Color,
    z: f32,
) {
    let path = SvgPath::parse(d);
    path.tessellate(mesh, dst_rect, src_rect, color, z);
}

/// Convenience: add a rounded rect outline that mimics a "stroke" effect.
pub fn _stroke_rounded(mesh: &mut Mesh, r: Rect, color: Color, _radius: Corners, _z: f32) {
    // Simple full-fill fallback (unused — kept for future stroke support).
    mesh.add_rect(r, color, _z);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Rect;

    #[test]
    fn parse_simple_path() {
        // Square: M 0 0 L 10 0 L 10 10 L 0 10 Z
        let p = SvgPath::parse("M 0 0 L 10 0 L 10 10 L 0 10 Z");
        assert_eq!(p.sub_paths.len(), 1);
        // 4 corners + closing point at start
        assert!(p.sub_paths[0].len() >= 4);
    }

    #[test]
    fn parse_relative_path() {
        let p = SvgPath::parse("m 0 0 l 10 0 l 0 10 l -10 0 z");
        assert_eq!(p.sub_paths.len(), 1);
    }

    #[test]
    fn parse_arc_path() {
        let p = SvgPath::parse("M 0 50 A 50 50 0 1 1 100 50 Z");
        assert_eq!(p.sub_paths.len(), 1);
    }

    #[test]
    fn parse_cubic_curve() {
        let p = SvgPath::parse("M 0 0 C 10 10 20 10 30 0");
        assert!(!p.sub_paths.is_empty());
    }

    #[test]
    fn tessellate_into_mesh() {
        let p = SvgPath::parse("M 0 0 L 10 0 L 10 10 L 0 10 Z");
        let mut mesh = Mesh::new();
        p.tessellate(
            &mut mesh,
            Rect::from_xywh(0.0, 0.0, 100.0, 100.0),
            Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
            Color::WHITE,
            0.0,
        );
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.indices.is_empty());
    }

    #[test]
    fn fill_svg_path_convenience() {
        let mut mesh = Mesh::new();
        fill_svg_path(
            &mut mesh,
            "M 0 0 L 24 0 L 24 24 L 0 24 Z",
            Rect::from_xywh(0.0, 0.0, 24.0, 24.0),
            Rect::from_xywh(0.0, 0.0, 24.0, 24.0),
            Color::BLACK,
            0.0,
        );
        assert!(!mesh.indices.is_empty());
    }
}
