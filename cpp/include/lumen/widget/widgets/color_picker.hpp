#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class ColorPicker : public Widget {
public:
    ColorPicker(Color color) : color_(color) { style_ = Style().w(64).h(32).rounded_md().border(1).border_color_(Color(203,213,225)).cursor_pointer().build(); }
    Color color() const { return color_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "ColorPicker"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, color_, style_.border_radius); ctx.painter.stroke_rect(rect, style_.border_color, 1.0f); }
private:
    ResolvedStyle style_; Color color_;
};
} // namespace lumen::widgets
