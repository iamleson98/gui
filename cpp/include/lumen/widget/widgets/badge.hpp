#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Badge : public Widget {
public:
    Badge(std::string text, Color color) : color_(color) { style_ = Style().px_2().py_1().rounded_full().build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Badge"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, color_, style_.border_radius); }
private:
    ResolvedStyle style_; Color color_;
};
} // namespace lumen::widgets
