#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Gauge : public Widget {
public:
    Gauge(float value) : value_(std::clamp(value,0.0f,1.0f)) { style_ = Style().w(120).h(120).build(); }
    float value() const { return value_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Gauge"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        float cx=rect.center().x, cy=rect.center().y, r=std::min(rect.width(),rect.height())*0.5f-8; if(r<=0) return;
        float start=3.0f*0.785398f, sweep=3.0f*1.570796f;
        for(int i=0; i<64; ++i) { float a=start+sweep*i/64.0f; if(a>start+sweep*value_) break;
            float px=cx+r*std::cos(a), py=cy+r*std::sin(a); ctx.painter.fill_rect(Rect::from_xywh(px-3,py-3,6,6), Color::TW_INDIGO_500); }
    }
private:
    ResolvedStyle style_; float value_;
};
} // namespace lumen::widgets
