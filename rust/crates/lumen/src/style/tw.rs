use crate::core::Color;
use crate::style::{
    Align, Corners, Cursor, Display, Edges, FlexDirection, Overflow, Style, TextAlign, TrackSize,
};

pub trait Tw: Sized {
    fn p(self, v: f32) -> Self;
    fn px(self, v: f32) -> Self;
    fn py(self, v: f32) -> Self;
    fn p_1(self) -> Self;
    fn p_2(self) -> Self;
    fn p_3(self) -> Self;
    fn p_4(self) -> Self;
    fn p_8(self) -> Self;
    fn px_2(self) -> Self;
    fn px_3(self) -> Self;
    fn px_4(self) -> Self;
    fn py_1(self) -> Self;
    fn py_2(self) -> Self;
    fn py_3(self) -> Self;
    fn m(self, v: f32) -> Self;
    fn rounded(self, v: f32) -> Self;
    fn rounded_md(self) -> Self;
    fn rounded_lg(self) -> Self;
    fn rounded_full(self) -> Self;
    fn bg(self, c: Color) -> Self;
    fn bg_primary(self) -> Self;
    fn bg_white(self) -> Self;
    fn text_color(self, c: Color) -> Self;
    fn text_white(self) -> Self;
    fn text_xs(self) -> Self;
    fn text_sm(self) -> Self;
    fn text_lg(self) -> Self;
    fn text_xl(self) -> Self;
    fn text_2xl(self) -> Self;
    fn font_bold(self) -> Self;
    fn font_medium(self) -> Self;
    fn flex(self) -> Self;
    fn flex_col(self) -> Self;
    fn grid(self) -> Self;
    fn hidden(self) -> Self;
    fn flex_grow(self, v: f32) -> Self;
    fn gap(self, v: f32) -> Self;
    fn gap_1(self) -> Self;
    fn gap_2(self) -> Self;
    fn gap_4(self) -> Self;
    fn items_center(self) -> Self;
    fn justify_center(self) -> Self;
    fn justify_between(self) -> Self;
    fn text_center(self) -> Self;
    fn w(self, v: f32) -> Self;
    fn h(self, v: f32) -> Self;
    fn w_full(self) -> Self;
    fn h_full(self) -> Self;
    fn border(self, w: f32) -> Self;
    fn border_color(self, c: Color) -> Self;
    fn cursor_pointer(self) -> Self;
    fn overflow_hidden(self) -> Self;
    fn shadow_md(self) -> Self;
    fn shadow_lg(self) -> Self;
    fn max_w(self, v: f32) -> Self;
    fn grid_cols(self, n: u32) -> Self;
}

macro_rules! impl_tw {
    () => {
        fn p(mut self, v: f32) -> Self {
            self.0.padding = Edges::all(v);
            self
        }
        fn px(mut self, v: f32) -> Self {
            self.0.padding.left = v;
            self.0.padding.right = v;
            self
        }
        fn py(mut self, v: f32) -> Self {
            self.0.padding.top = v;
            self.0.padding.bottom = v;
            self
        }
        fn p_1(self) -> Self {
            self.p(4.0)
        }
        fn p_2(self) -> Self {
            self.p(8.0)
        }
        fn p_3(self) -> Self {
            self.p(12.0)
        }
        fn p_4(self) -> Self {
            self.p(16.0)
        }
        fn p_8(self) -> Self {
            self.p(32.0)
        }
        fn px_2(self) -> Self {
            self.px(8.0)
        }
        fn px_3(self) -> Self {
            self.px(12.0)
        }
        fn px_4(self) -> Self {
            self.px(16.0)
        }
        fn py_1(self) -> Self {
            self.py(4.0)
        }
        fn py_2(self) -> Self {
            self.py(8.0)
        }
        fn py_3(self) -> Self {
            self.py(12.0)
        }
        fn m(mut self, v: f32) -> Self {
            self.0.margin = Edges::all(v);
            self
        }
        fn rounded(mut self, v: f32) -> Self {
            self.0.border_radius = Corners::all(v);
            self
        }
        fn rounded_md(self) -> Self {
            self.rounded(6.0)
        }
        fn rounded_lg(self) -> Self {
            self.rounded(10.0)
        }
        fn rounded_full(mut self) -> Self {
            self.0.border_radius = Corners::all(f32::INFINITY);
            self
        }
        fn bg(mut self, c: Color) -> Self {
            self.0.background = c;
            self
        }
        fn bg_primary(self) -> Self {
            self.bg(Color::TW_INDIGO_500)
        }
        fn bg_white(self) -> Self {
            self.bg(Color::WHITE)
        }
        fn text_color(mut self, c: Color) -> Self {
            self.0.color = c;
            self
        }
        fn text_white(self) -> Self {
            self.text_color(Color::WHITE)
        }
        fn text_xs(mut self) -> Self {
            self.0.font_size = 10.0;
            self
        }
        fn text_sm(mut self) -> Self {
            self.0.font_size = 12.0;
            self
        }
        fn text_lg(mut self) -> Self {
            self.0.font_size = 18.0;
            self
        }
        fn text_xl(mut self) -> Self {
            self.0.font_size = 22.0;
            self
        }
        fn text_2xl(mut self) -> Self {
            self.0.font_size = 28.0;
            self
        }
        fn font_bold(mut self) -> Self {
            self.0.font_weight = 700;
            self
        }
        fn font_medium(mut self) -> Self {
            self.0.font_weight = 500;
            self
        }
        fn flex(mut self) -> Self {
            self.0.display = Display::Flex;
            self
        }
        fn flex_col(mut self) -> Self {
            self.0.display = Display::Flex;
            self.0.flex_direction = FlexDirection::Column;
            self
        }
        fn grid(mut self) -> Self {
            self.0.display = Display::Grid;
            self
        }
        fn hidden(mut self) -> Self {
            self.0.display = Display::None_;
            self
        }
        fn flex_grow(mut self, v: f32) -> Self {
            self.0.flex_grow = v;
            self
        }
        fn gap(mut self, v: f32) -> Self {
            self.0.gap = v;
            self
        }
        fn gap_1(self) -> Self {
            self.gap(4.0)
        }
        fn gap_2(self) -> Self {
            self.gap(8.0)
        }
        fn gap_4(self) -> Self {
            self.gap(16.0)
        }
        fn items_center(mut self) -> Self {
            self.0.align_items = Align::Center;
            self
        }
        fn justify_center(mut self) -> Self {
            self.0.justify_content = Align::Center;
            self
        }
        fn justify_between(mut self) -> Self {
            self.0.justify_content = Align::SpaceBetween;
            self
        }
        fn text_center(mut self) -> Self {
            self.0.text_align = TextAlign::Center;
            self
        }
        fn w(mut self, v: f32) -> Self {
            self.0.width = Some(v);
            self
        }
        fn h(mut self, v: f32) -> Self {
            self.0.height = Some(v);
            self
        }
        fn w_full(mut self) -> Self {
            self.0.width = Some(f32::INFINITY);
            self.0.flex_grow = 1.0;
            self
        }
        fn h_full(mut self) -> Self {
            self.0.height = Some(f32::INFINITY);
            self.0.flex_grow = 1.0;
            self
        }
        fn border(mut self, w: f32) -> Self {
            self.0.border_width = Edges::all(w);
            self
        }
        fn border_color(mut self, c: Color) -> Self {
            self.0.border_color = c;
            self
        }
        fn cursor_pointer(mut self) -> Self {
            self.0.cursor = Cursor::Pointer;
            self
        }
        fn overflow_hidden(mut self) -> Self {
            self.0.overflow = Overflow::Hidden;
            self
        }
        fn shadow_md(mut self) -> Self {
            self.0.background = self.0.background;
            self
        }
        fn shadow_lg(mut self) -> Self {
            self.0.background = self.0.background;
            self
        }
        fn max_w(mut self, v: f32) -> Self {
            self.0.width = Some(v);
            self
        }
        fn grid_cols(mut self, n: u32) -> Self {
            self.0.display = Display::Grid;
            self.0.grid_template_columns = (0..n).map(|_| TrackSize::Fr(1.0)).collect();
            self
        }
    };
}

impl Tw for Style {
    impl_tw!();
}

pub trait Tailwind {
    fn apply(self, classes: &str) -> Self;
}
impl Tailwind for Style {
    fn apply(self, classes: &str) -> Self {
        let mut s = self;
        for c in classes.split_whitespace() {
            use crate::style::Tw as _;
            s = match c {
                "p-2" => s.p(8.0),
                "p-3" => s.p(12.0),
                "p-4" => s.p(16.0),
                "px-4" => s.px(16.0),
                "py-2" => s.py(8.0),
                "rounded-md" => s.rounded_md(),
                "rounded-lg" => s.rounded_lg(),
                "bg-white" => s.bg_white(),
                "bg-primary" => s.bg_primary(),
                "bg-indigo-500" => s.bg(Color::TW_INDIGO_500),
                "text-white" => s.text_white(),
                "text-sm" => s.text_sm(),
                "font-bold" => s.font_bold(),
                "font-medium" => s.font_medium(),
                "flex" => s.flex(),
                "flex-col" => s.flex_col(),
                "items-center" => s.items_center(),
                "justify-center" => s.justify_center(),
                "justify-between" => s.justify_between(),
                "text-center" => s.text_center(),
                "w-full" => s.w_full(),
                "h-full" => s.h_full(),
                "cursor-pointer" => s.cursor_pointer(),
                "shadow-md" => s.shadow_md(),
                _ => s,
            };
        }
        s
    }
}
