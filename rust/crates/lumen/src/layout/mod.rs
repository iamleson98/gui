use crate::core::{Id, Vec2};
use crate::style::{Align, Display, FlexDirection, ResolvedStyle, TrackSize};

#[derive(Clone, Copy, Debug)]
pub struct Constraints {
    pub min: Vec2,
    pub max: Vec2,
}
impl Constraints {
    pub const UNBOUNDED: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::new(f32::INFINITY, f32::INFINITY),
    };
    pub fn tight(size: Vec2) -> Self {
        Self {
            min: size,
            max: size,
        }
    }
    pub fn loose(self) -> Self {
        Self {
            min: Vec2::ZERO,
            max: self.max,
        }
    }
    pub fn constrain(self, size: Vec2) -> Vec2 {
        Vec2::new(
            size.x.clamp(self.min.x, self.max.x),
            size.y.clamp(self.min.y, self.max.y),
        )
    }
}

pub type MeasureFn = Box<dyn Fn(Constraints) -> Vec2 + Send + Sync>;

pub struct LayoutNode<'a> {
    pub id: Id,
    pub style: &'a ResolvedStyle,
    pub children: Vec<LayoutNode<'a>>,
    pub measure: Option<&'a MeasureFn>,
}

#[derive(Clone, Debug)]
pub struct LayoutRect {
    pub id: Id,
    pub rect: crate::core::Rect,
    pub children: Vec<LayoutRect>,
}

pub fn arrange(node: &LayoutNode<'_>, viewport: Vec2) -> LayoutRect {
    let c = Constraints::tight(viewport).loose();
    let (size, children) = measure_and_arrange(node, c);
    LayoutRect {
        id: node.id,
        rect: crate::core::Rect::from_min_size(Vec2::ZERO, size),
        children,
    }
}

fn measure_and_arrange(node: &LayoutNode<'_>, c: Constraints) -> (Vec2, Vec<LayoutRect>) {
    let s = node.style;
    let mw = s.margin.left
        + s.margin.right
        + s.border_width.left
        + s.border_width.right
        + s.padding.left
        + s.padding.right;
    let mh = s.margin.top
        + s.margin.bottom
        + s.border_width.top
        + s.border_width.bottom
        + s.padding.top
        + s.padding.bottom;
    let outer = Vec2::new(mw, mh);
    let inner_c = Constraints {
        min: Vec2::new((c.min.x - outer.x).max(0.0), (c.min.y - outer.y).max(0.0)),
        max: Vec2::new((c.max.x - outer.x).max(0.0), (c.max.y - outer.y).max(0.0)),
    };
    if node.children.is_empty() {
        let size = apply_sizing(s, Vec2::ZERO, inner_c);
        return (c.constrain(size + outer), Vec::new());
    }
    match s.display {
        Display::None_ => (Vec2::ZERO, Vec::new()),
        Display::Block | Display::Flex => layout_flex(node, inner_c, outer),
        Display::Grid => layout_grid(node, inner_c, outer),
    }
}

fn apply_sizing(s: &ResolvedStyle, intrinsic: Vec2, c: Constraints) -> Vec2 {
    let mut size = intrinsic;
    if let Some(w) = s.width {
        if !w.is_infinite() {
            size.x = w;
        }
    }
    if let Some(h) = s.height {
        if !h.is_infinite() {
            size.y = h;
        }
    }
    c.constrain(size)
}

fn layout_flex(
    node: &LayoutNode<'_>,
    inner_c: Constraints,
    outer_pad: Vec2,
) -> (Vec2, Vec<LayoutRect>) {
    let horizontal = matches!(
        node.style.flex_direction,
        FlexDirection::Row | FlexDirection::RowReverse
    );
    let child_c = if horizontal {
        Constraints {
            min: Vec2::ZERO,
            max: Vec2::new(f32::INFINITY, inner_c.max.y),
        }
    } else {
        Constraints {
            min: Vec2::ZERO,
            max: Vec2::new(inner_c.max.x, f32::INFINITY),
        }
    };
    let mut entries: Vec<(f32, f32, Vec<LayoutRect>)> = Vec::new();
    let mut total_basis = 0.0f32;
    let mut total_grow = 0.0f32;
    for child in &node.children {
        let (size, sub) = measure_and_arrange(child, child_c);
        let basis = child
            .style
            .flex_basis
            .unwrap_or(if horizontal { size.x } else { size.y });
        let cross = if horizontal { size.y } else { size.x };
        total_basis += basis;
        total_grow += child.style.flex_grow;
        entries.push((basis, cross, sub));
    }
    let main_avail = if horizontal {
        inner_c.max.x
    } else {
        inner_c.max.y
    };
    let gap_total = if entries.is_empty() {
        0.0
    } else {
        node.style.gap * (entries.len() - 1) as f32
    };
    let free = (main_avail - total_basis - gap_total).max(0.0);
    let grow_safe = if total_grow > 0.0 { total_grow } else { 1.0 };
    let final_main: Vec<f32> = entries
        .iter()
        .enumerate()
        .map(|(i, (basis, _, _))| {
            if total_grow > 0.0 {
                basis + free * (node.children[i].style.flex_grow / grow_safe)
            } else {
                *basis
            }
        })
        .collect();
    let max_cross = entries
        .iter()
        .map(|(_, c, _)| *c)
        .fold(0.0f32, f32::max)
        .min(if horizontal {
            inner_c.max.y
        } else {
            inner_c.max.x
        });
    let align = node.style.align_items;
    let mut layouts = Vec::new();
    let mut pos = 0.0f32;
    for (i, (_, cross, _)) in entries.iter().enumerate() {
        let main = final_main[i];
        let cross_pos = match align {
            Align::Center => (max_cross - cross) * 0.5,
            Align::End => max_cross - cross,
            _ => 0.0,
        };
        let (x, y, w, h) = if horizontal {
            (
                pos,
                cross_pos,
                main,
                if matches!(align, Align::Stretch) {
                    max_cross
                } else {
                    *cross
                },
            )
        } else {
            (
                cross_pos,
                pos,
                if matches!(align, Align::Stretch) {
                    max_cross
                } else {
                    *cross
                },
                main,
            )
        };
        let mut sub = entries[i].2.clone();
        for s2 in sub.iter_mut() {
            s2.rect = s2.rect.translate(Vec2::new(x, y));
        }
        layouts.push(LayoutRect {
            id: node.children[i].id,
            rect: crate::core::Rect::from_xywh(x, y, w, h),
            children: sub,
        });
        pos += main + node.style.gap;
    }
    let intrinsic = if horizontal {
        Vec2::new(final_main.iter().sum::<f32>() + gap_total, max_cross)
    } else {
        Vec2::new(max_cross, final_main.iter().sum::<f32>() + gap_total)
    };
    let size = apply_sizing(node.style, intrinsic, inner_c);
    (size + outer_pad, layouts)
}

fn layout_grid(
    node: &LayoutNode<'_>,
    inner_c: Constraints,
    outer_pad: Vec2,
) -> (Vec2, Vec<LayoutRect>) {
    let cols = &node.style.grid_template_columns;
    if cols.is_empty() {
        return layout_flex(node, inner_c, outer_pad);
    }
    let n_cols = cols.len();
    let avail_w = inner_c.max.x;
    let total_fr: f32 = cols
        .iter()
        .map(|t| match t {
            TrackSize::Fr(v) => *v,
            _ => 0.0,
        })
        .sum::<f32>()
        .max(1.0);
    let fr_w = (avail_w - node.style.gap * (n_cols - 1) as f32).max(0.0) / total_fr;
    let col_widths: Vec<f32> = cols
        .iter()
        .map(|t| match t {
            TrackSize::Fr(v) => fr_w * v,
            TrackSize::Px(v) => *v,
            _ => 0.0,
        })
        .collect();
    let n_rows = (node.children.len() + n_cols - 1) / n_cols.max(1);
    let mut row_heights = vec![0.0f32; n_rows];
    let mut layouts = Vec::new();
    for (i, child) in node.children.iter().enumerate() {
        let r = i / n_cols.max(1);
        let c = i % n_cols.max(1);
        let x: f32 = (0..c).map(|j| col_widths[j]).sum::<f32>() + node.style.gap * c as f32;
        let y = node.style.gap * r as f32;
        let w = col_widths[c];
        let (size, sub) = measure_and_arrange(
            child,
            Constraints {
                min: Vec2::ZERO,
                max: Vec2::new(w, f32::INFINITY),
            },
        );
        row_heights[r] = row_heights[r].max(size.y);
        let mut sub = sub;
        for s2 in sub.iter_mut() {
            s2.rect = s2.rect.translate(Vec2::new(x, y));
        }
        layouts.push(LayoutRect {
            id: child.id,
            rect: crate::core::Rect::from_xywh(x, y, w, size.y),
            children: sub,
        });
    }
    let mut row_offsets = vec![0.0f32; n_rows];
    for r in 1..n_rows {
        row_offsets[r] = row_offsets[r - 1] + row_heights[r - 1] + node.style.gap;
    }
    for (i, l) in layouts.iter_mut().enumerate() {
        let r = i / n_cols.max(1);
        let dy = row_offsets[r] - node.style.gap * r as f32;
        l.rect = l.rect.translate(Vec2::new(0.0, dy));
        for s2 in l.children.iter_mut() {
            s2.rect = s2.rect.translate(Vec2::new(0.0, dy));
        }
    }
    let total_w =
        col_widths.iter().sum::<f32>() + node.style.gap * (n_cols.saturating_sub(1)) as f32;
    let total_h =
        row_heights.iter().sum::<f32>() + node.style.gap * (n_rows.saturating_sub(1)) as f32;
    let size = apply_sizing(node.style, Vec2::new(total_w, total_h), inner_c);
    (size + outer_pad, layouts)
}
