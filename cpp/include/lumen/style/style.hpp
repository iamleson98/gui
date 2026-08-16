#pragma once
#include "lumen/core/color.hpp"
#include "lumen/core/vec2.hpp"
#include <string>
#include <vector>
#include <optional>
#include <functional>
namespace lumen {

struct Edges { float top=0, right=0, bottom=0, left=0; static Edges all(float v) { return {v,v,v,v}; } };
struct Corners { float top_left=0, top_right=0, bottom_right=0, bottom_left=0; static Corners all(float v) { return {v,v,v,v}; } };
enum class Display { Block, Flex, Grid, None_ };
enum class FlexDirection { Row, RowReverse, Column, ColumnReverse };
enum class Align { Start, Center, End, Stretch, SpaceBetween, SpaceAround, Auto };
enum class TextAlign { Left, Center, Right, Justify };
enum class Cursor { Default, Pointer, Text, Crosshair, NotAllowed, ResizeNS, ResizeEW, ResizeNESW, ResizeNWSE };
enum class Overflow { Visible, Hidden, Scroll, Clip };
enum class TrackSizeType { Auto, Fr, Px, MinContent, MaxContent };
struct TrackSize { TrackSizeType type=TrackSizeType::Auto; float value=0; static TrackSize fr(float v) { return {TrackSizeType::Fr, v}; } };

struct ResolvedStyle {
    Edges margin, padding, border_width; Color border_color=Color::TRANSPARENT; Corners border_radius;
    Color background=Color::TRANSPARENT, color=Color::BLACK;
    float font_size=14.0f; uint16_t font_weight=400; TextAlign text_align=TextAlign::Left;
    Display display=Display::Block; FlexDirection flex_direction=FlexDirection::Row;
    Align justify_content=Align::Start, align_items=Align::Stretch;
    float flex_grow=0, flex_shrink=1, gap=0; std::optional<float> flex_basis, width, height, min_width, min_height, max_width, max_height;
    float opacity=1.0f; Cursor cursor=Cursor::Default; Overflow overflow=Overflow::Visible;
    std::vector<TrackSize> grid_template_columns, grid_template_rows;
};

class Style {
public:
    Style() = default;
    ResolvedStyle& get() { return s_; }
    const ResolvedStyle& get() const { return s_; }
    ResolvedStyle build() const { return s_; }

    Style& p(float v) { s_.padding = Edges::all(v); return *this; }
    Style& px(float v) { s_.padding.left=v; s_.padding.right=v; return *this; }
    Style& py(float v) { s_.padding.top=v; s_.padding.bottom=v; return *this; }
    Style& p_1() { return p(4); } Style& p_2() { return p(8); } Style& p_3() { return p(12); } Style& p_4() { return p(16); } Style& p_6() { return p(24); } Style& p_8() { return p(32); }
    Style& px_2() { return px(8); } Style& px_3() { return px(12); } Style& px_4() { return px(16); } Style& px_6() { return px(24); } Style& px_8() { return px(32); }
    Style& py_1() { return py(4); } Style& py_2() { return py(8); } Style& py_3() { return py(12); } Style& py_4() { return py(16); }
    Style& m(float v) { s_.margin = Edges::all(v); return *this; }
    Style& rounded(float v) { s_.border_radius = Corners::all(v); return *this; }
    Style& rounded_md() { return rounded(6); } Style& rounded_lg() { return rounded(10); } Style& rounded_xl() { return rounded(14); } Style& rounded_full() { s_.border_radius = Corners::all(INFINITY); return *this; }
    Style& bg(Color c) { s_.background = c; return *this; }
    Style& bg_primary() { return bg(Color::TW_INDIGO_600); } Style& bg_white() { return bg(Color::WHITE); }
    Style& bg_surface() { return bg(Color::WHITE); } Style& bg_muted() { return bg(Color::TW_SLATE_100); }
    Style& text_color(Color c) { s_.color = c; return *this; } Style& text_white() { return text_color(Color::WHITE); } Style& text_muted() { return text_color(Color::TW_SLATE_500); }
    Style& text_xs() { s_.font_size=10; return *this; } Style& text_sm() { s_.font_size=12; return *this; }
    Style& text_base() { s_.font_size=14; return *this; }
    Style& text_lg() { s_.font_size=18; return *this; } Style& text_xl() { s_.font_size=22; return *this; } Style& text_2xl() { s_.font_size=28; return *this; } Style& text_3xl() { s_.font_size=36; return *this; }
    Style& font_bold() { s_.font_weight=700; return *this; } Style& font_medium() { s_.font_weight=500; return *this; }
    Style& flex() { s_.display=Display::Flex; return *this; } Style& flex_col() { s_.display=Display::Flex; s_.flex_direction=FlexDirection::Column; return *this; }
    Style& grid() { s_.display=Display::Grid; return *this; } Style& hidden() { s_.display=Display::None_; return *this; }
    Style& flex_grow_(float v) { s_.flex_grow=v; return *this; }
    Style& gap(float v) { s_.gap=v; return *this; } Style& gap_1() { return gap(4); } Style& gap_2() { return gap(8); } Style& gap_4() { return gap(16); } Style& gap_6() { return gap(24); } Style& gap_8() { return gap(32); }
    Style& items_center() { s_.align_items=Align::Center; return *this; }
    Style& items_start() { s_.align_items=Align::Start; return *this; }
    Style& items_end() { s_.align_items=Align::End; return *this; }
    Style& items_stretch() { s_.align_items=Align::Stretch; return *this; }
    Style& justify_center() { s_.justify_content=Align::Center; return *this; }
    Style& justify_start() { s_.justify_content=Align::Start; return *this; }
    Style& justify_end() { s_.justify_content=Align::End; return *this; }
    Style& justify_between() { s_.justify_content=Align::SpaceBetween; return *this; }
    Style& text_center() { s_.text_align=TextAlign::Center; return *this; }
    Style& text_left() { s_.text_align=TextAlign::Left; return *this; }
    Style& text_right() { s_.text_align=TextAlign::Right; return *this; }
    Style& w(float v) { s_.width=v; return *this; } Style& h(float v) { s_.height=v; return *this; }
    Style& min_w(float v) { s_.min_width=v; return *this; } Style& min_h(float v) { s_.min_height=v; return *this; }
    Style& w_full() { s_.width=INFINITY; s_.flex_grow=1; return *this; } Style& h_full() { s_.height=INFINITY; s_.flex_grow=1; return *this; }
    Style& border(float w) { s_.border_width=Edges::all(w); return *this; } Style& border_color_(Color c) { s_.border_color=c; return *this; }
    Style& cursor_pointer() { s_.cursor=Cursor::Pointer; return *this; } Style& overflow_hidden() { s_.overflow=Overflow::Hidden; return *this; }
    Style& shadow_md() { return *this; } Style& shadow_lg() { return *this; } Style& max_w(float v) { s_.max_width=v; return *this; }
    Style& grid_cols(uint32_t n) { s_.display=Display::Grid; s_.grid_template_columns.clear(); for(uint32_t i=0;i<n;++i) s_.grid_template_columns.push_back(TrackSize::fr(1.0f)); return *this; }
    Style& apply(const std::string& classes);
private:
    ResolvedStyle s_;
};

/// The color palette for a theme. Synced with Rust style/theme.rs.
struct Palette {
    Color bg, surface, surface_elevated, muted;
    Color primary, primary_hover, primary_active, accent;
    Color text, text_muted, border, shadow;
    Color danger, success, warning, info;
};
struct Theme {
    enum class Kind { Light, Dark, HighContrast };
    Kind kind=Kind::Light; Palette palette; float density=1.0f;
    static Theme light(); static Theme dark();
};
} // namespace lumen
