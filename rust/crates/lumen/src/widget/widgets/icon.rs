use crate::core::{Color, Rect, Vec2};
use crate::style::{ResolvedStyle, Style, Tw};
use crate::widget::{PaintCtx, Widget};
use crate::widget::widgets::icons::IconKind;

/// The viewBox source rectangle for built-in icons (24×24).
pub const ICON_VIEWBOX: Rect = Rect { min: Vec2::new(0.0, 0.0), max: Vec2::new(24.0, 24.0) };

/// A widget that renders a single SVG icon path. Built-in icons come from
/// `IconKind`; custom SVG path strings can be supplied via `Icon::from_path`.
///
/// The icon is drawn as a solid fill (no stroke) using `with_color(...)`.
/// It scales to fit `style.width` × `style.height` (defaulting to the size
/// passed at construction).
pub struct Icon {
    style: ResolvedStyle,
    path: &'static str,
    color: Color,
    size: f32,
}

impl Icon {
    /// Create an Icon from a built-in `IconKind`.
    pub fn new(kind: IconKind, size: f32) -> Self {
        Self {
            style: Style::new().w(size).h(size).build(),
            path: kind.path(),
            color: Color::BLACK,
            size,
        }
    }

    /// Create an Icon from a raw SVG path string (24×24 viewBox assumed).
    pub fn from_path(path: &'static str, size: f32) -> Self {
        Self {
            style: Style::new().w(size).h(size).build(),
            path,
            color: Color::BLACK,
            size,
        }
    }

    pub fn with_color(mut self, c: Color) -> Self { self.color = c; self.style.color = c; self }
    pub fn with_style(mut self, s: ResolvedStyle) -> Self { self.style = s; self }
    pub fn path(&self) -> &'static str { self.path }
    pub fn size(&self) -> f32 { self.size }
}

impl Widget for Icon {
    fn style(&self) -> &ResolvedStyle { &self.style }
    fn debug_name(&self) -> &'static str { "Icon" }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn paint(&self, ctx: &mut PaintCtx<'_>, rect: &Rect) {
        // Center the icon within the rect (rect may differ from size due to
        // flex layout).
        let s = self.size.min(rect.width()).min(rect.height());
        let cx = rect.center().x;
        let cy = rect.center().y;
        let dst = Rect::from_xywh(cx - s * 0.5, cy - s * 0.5, s, s);
        ctx.painter.fill_svg(self.path, dst, ICON_VIEWBOX, self.color);
    }
}
