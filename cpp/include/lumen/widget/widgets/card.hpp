#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Card : public Widget {
public:
    Card(std::vector<Element> body) : body_(std::move(body)) { style_ = Style().p_4().rounded_lg().bg_white().shadow_md().flex_col().gap_2().build(); }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return body_; }
    std::vector<Element>& children_mut() override { return body_; }
    std::string debug_name() const override { return "Card"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_; std::vector<Element> body_;
};
} // namespace lumen::widgets
