#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Rating : public Widget {
public:
    Rating(uint32_t max, float value) : max_(max), value_(std::clamp(value,0.0f,(float)max)) { style_ = Style().flex().gap_1().build(); }
    float value() const { return value_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Rating"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        float sw=rect.width()/max_; for(uint32_t i=0; i<max_; ++i) { float x=rect.min.x+sw*i;
            ctx.painter.fill_rounded_rect(Rect::from_xywh(x,rect.min.y,sw,rect.height()), Color(226,232,240), Corners::all(2));
            float f=std::clamp(value_-(float)i,0.0f,1.0f); if(f>0) ctx.painter.fill_rounded_rect(Rect::from_xywh(x,rect.min.y,sw*f,rect.height()), Color::TW_AMBER_500, Corners::all(2)); } }
    EventResult on_event(EventCtx& ctx, const Event& event) override { if(event.kind==Event::Kind::PointerDown) { float t=std::clamp((event.pos.x-ctx.current_rect.min.x)/ctx.current_rect.width(),0.0f,1.0f); value_=std::ceil(t*max_); ctx.state.request_redraw(); return EventResult::HandledAndRedraw; } return EventResult::Ignored; }
private:
    ResolvedStyle style_; uint32_t max_; float value_;
};
} // namespace lumen::widgets
