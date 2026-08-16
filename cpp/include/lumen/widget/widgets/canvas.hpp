#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Canvas : public Widget {
public:
    Canvas() { style_ = Style().w_full().h_full().bg_white().build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Canvas"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_;
};
} // namespace lumen::widgets
