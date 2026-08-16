#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/widget/widgets/icons.hpp"
#include "lumen/core/color.hpp"
#include <cmath>

namespace lumen::widgets {

/// The viewBox source rectangle for built-in icons (24x24).
inline Rect icon_viewbox() { return Rect(Vec2(0,0), Vec2(24,24)); }

/// A widget that renders a single SVG icon path. Built-in icons come from
/// `IconKind`; custom SVG path strings can be supplied via the constructor
/// that takes a string.
class Icon : public Widget {
public:
    /// Create an Icon from a built-in `IconKind`.
    Icon(IconKind kind, float size) : path_(icon_path(kind)), color_(Color::BLACK), size_(size) {
        style_ = Style().w(size).h(size).build();
    }
    /// Create an Icon from a raw SVG path string (24x24 viewBox assumed).
    Icon(const std::string& path, float size) : path_(path), color_(Color::BLACK), size_(size) {
        style_ = Style().w(size).h(size).build();
    }
    Icon& with_color(Color c) { color_ = c; style_.color = c; return *this; }
    Icon& with_style(ResolvedStyle s) { style_ = s; return *this; }
    const std::string& path() const { return path_; }
    float size() const { return size_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Icon"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        // Center the icon within the rect (may differ from size due to flex).
        float s = std::min(size_, std::min(rect.width(), rect.height()));
        float cx = (rect.min.x + rect.max.x) * 0.5f;
        float cy = (rect.min.y + rect.max.y) * 0.5f;
        Rect dst = Rect::from_xywh(cx - s*0.5f, cy - s*0.5f, s, s);
        ctx.painter.fill_svg(path_, dst, icon_viewbox(), color_);
    }
private:
    ResolvedStyle style_;
    std::string path_;
    Color color_;
    float size_;
};

} // namespace lumen::widgets
