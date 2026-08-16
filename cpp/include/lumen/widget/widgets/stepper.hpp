#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct Step { std::string label; bool optional=false; Step(std::string l) : label(std::move(l)) {} };
class Stepper : public Widget {
public:
    Stepper(std::vector<Step> steps) : steps_(std::move(steps)) { style_ = Style().flex().build(); }
    size_t current() const { return current_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Stepper"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        size_t n=steps_.size(); if(n==0) return; float sw=rect.width()/n, r=14;
        for(size_t i=0; i<n; ++i) { float cx=rect.min.x+sw*(i+0.5f), cy=rect.min.y+r+4;
            auto bg = i<current_ ? Color::TW_EMERALD_500 : (i==current_ ? Color::TW_INDIGO_500 : Color(203,213,225));
            ctx.painter.fill_rounded_rect(Rect::from_xywh(cx-r,cy-r,r*2,r*2), bg, Corners::all(r)); }
    }
private:
    ResolvedStyle style_; std::vector<Step> steps_; size_t current_=0;
};
} // namespace lumen::widgets
