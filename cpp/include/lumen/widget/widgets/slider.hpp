#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

struct SliderChanged { float value; };
class Slider : public Widget {
public:
    Slider(float min, float max, float value) : min_(min), max_(max), value_(std::clamp(value,min,max)) { style_ = Style().h(8).w_full().rounded_md().bg(Color(226,232,240)).cursor_pointer().build(); }
    float value() const { return value_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Slider"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); float t=(value_-min_)/std::max(1e-6f, max_-min_); ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width()*t, rect.height()), Color::TW_INDIGO_500, style_.border_radius); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { dragging_=true; ctx.state.capture_pointer(ctx.current_id); float t=std::clamp((event.pos.x-ctx.current_rect.min.x)/ctx.current_rect.width(),0.0f,1.0f); value_=min_+t*(max_-min_); return EventResult::HandledAndRedraw; }
        if(event.kind==Event::Kind::PointerUp) { if(dragging_) { dragging_=false; ctx.state.release_pointer(); return EventResult::HandledAndRedraw; } }
        return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; float min_, max_, value_; bool dragging_=false;
};
} // namespace lumen::widgets
