#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Spinner : public Widget {
public:
    Spinner(float size=24) : size_(size) { style_ = Style().w(size).h(size).build(); }
    void tick(float dt) { angle_ = std::fmod(angle_ + dt * 6.2831853f, 6.2831853f); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Spinner"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        float cx=rect.center().x, cy=rect.center().y, r=size_*0.5f-3.0f; if(r<=0) return;
        for(int i=0; i<12; ++i) { float t=i/12.0f, a=angle_+t*6.2831853f; uint8_t alpha=(uint8_t)((1-t)*255);
            float px=cx+r*std::cos(a), py=cy+r*std::sin(a); ctx.painter.fill_rect(Rect::from_xywh(px-2,py-2,4,4), Color::TW_INDIGO_500.with_alpha(alpha)); }
    }
private:
    ResolvedStyle style_; float size_, angle_=0;
};
} // namespace lumen::widgets
