mod theme;
mod tw;
use crate::core::Color;
pub use theme::{Theme, ThemeKind};
pub use tw::{Tailwind, Tw};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}
impl Edges {
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };
    pub const fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Corners {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}
impl Corners {
    pub const ZERO: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };
    pub const fn all(v: f32) -> Self {
        Self {
            top_left: v,
            top_right: v,
            bottom_right: v,
            bottom_left: v,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Block,
    Flex,
    Grid,
    None_,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
    Auto,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Cursor {
    #[default]
    Default,
    Pointer,
    Text,
    Crosshair,
    NotAllowed,
    ResizeNS,
    ResizeEW,
    ResizeNESW,
    ResizeNWSE,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Clip,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TrackSize {
    Auto,
    Fr(f32),
    Px(f32),
    MinContent,
    MaxContent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedStyle {
    pub margin: Edges,
    pub padding: Edges,
    pub border_width: Edges,
    pub border_color: Color,
    pub border_radius: Corners,
    pub background: Color,
    pub color: Color,
    pub font_size: f32,
    pub font_weight: u16,
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub justify_content: Align,
    pub align_items: Align,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub gap: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub opacity: f32,
    pub cursor: Cursor,
    pub overflow: Overflow,
    pub text_align: TextAlign,
    pub grid_template_columns: Vec<TrackSize>,
}
impl Default for ResolvedStyle {
    fn default() -> Self {
        Self {
            margin: Edges::ZERO,
            padding: Edges::ZERO,
            border_width: Edges::ZERO,
            border_color: Color::TRANSPARENT,
            border_radius: Corners::ZERO,
            background: Color::TRANSPARENT,
            color: Color::BLACK,
            font_size: 14.0,
            font_weight: 400,
            display: Display::Block,
            flex_direction: FlexDirection::Row,
            justify_content: Align::Start,
            align_items: Align::Stretch,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            gap: 0.0,
            width: None,
            height: None,
            opacity: 1.0,
            cursor: Cursor::Default,
            overflow: Overflow::Visible,
            text_align: TextAlign::Left,
            grid_template_columns: Vec::new(),
        }
    }
}
#[derive(Default, Clone, Debug)]
pub struct Style(pub ResolvedStyle);
impl Style {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self) -> ResolvedStyle {
        self.0
    }
}
