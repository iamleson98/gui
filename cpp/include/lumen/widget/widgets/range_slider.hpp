#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class RangeSlider : public Widget {
public:
    RangeSlider(float min, float max, float start, float end) : min_(min), max_(max), start_(std::clamp(start,min,max)), end_(std::clamp(end,start,max)) { style_ = Style().h(8).w_full().rounded_full().bg(Color(226,232,240)).cursor_pointer().build(); }
    float start() const { return start_; } float end() const { return end_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "RangeSlider"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius);
        float st=(start_-min_)/std::max(1e-6f,max_-min_), en=(end_-min_)/std::max(1e-6f,max_-min_);
        ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x+rect.width()*st, rect.min.y, rect.width()*(en-st), rect.height()), Color::TW_INDIGO_500, style_.border_radius);
    }
private:
    ResolvedStyle style_; float min_, max_, start_, end_;
};
} // namespace lumen::widgets
