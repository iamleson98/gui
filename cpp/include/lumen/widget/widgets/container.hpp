#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Container : public Widget {
public:
    Container(ResolvedStyle s, std::vector<Element> children = {}) : style_(s), children_(std::move(children)) {}
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return children_; }
    std::vector<Element>& children_mut() override { return children_; }
    std::string debug_name() const override { return "Container"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { if(style_.background.a > 0) ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_; std::vector<Element> children_;
};
} // namespace lumen::widgets
