#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Skeleton : public Widget {
public:
    Skeleton() { style_ = Style().w_full().h(16).rounded_md().bg(Color(226,232,240)).build(); }
    void tick(float dt) { progress_ = std::fmod(progress_ + dt*0.5f, 1.0f); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Skeleton"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius);
        float sw=rect.width()*0.3, sx=rect.min.x+(rect.width()+sw)*progress_-sw*0.5;
        auto sr = Rect::from_xywh(sx, rect.min.y, sw, rect.height()).intersect(rect);
        if(sr.width()>0) ctx.painter.fill_rounded_rect(sr, Color(255,255,255,80), style_.border_radius);
    }
private:
    ResolvedStyle style_; float progress_=0;
};
} // namespace lumen::widgets
