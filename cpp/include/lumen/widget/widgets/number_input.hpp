#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class NumberInput : public Widget {
public:
    NumberInput(double value) : value_(value) { style_ = Style().px_2().py_1().rounded_md().bg_white().border(1).border_color_(Color(203,213,225)).text_sm().build(); }
    NumberInput& with_range(double min, double max) { min_=min; max_=max; value_=std::clamp(value_,min,max); return *this; }
    NumberInput& with_step(double step) { step_=step; return *this; }
    double value() const { return value_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "NumberInput"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); auto b=focused_?Color::TW_INDIGO_500:style_.border_color; ctx.painter.stroke_rect(rect, b, 1.0f); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { if(event.pos.x >= ctx.current_rect.max.x-16) { value_ += (event.pos.y < ctx.current_rect.center().y ? 1 : -1) * step_; value_ = std::clamp(value_, min_, max_); return EventResult::HandledAndRedraw; } focused_=true; return EventResult::Handled; }
        if(event.kind==Event::Kind::KeyDown && focused_) { if(event.key==KeyCode::ArrowUp) { value_=std::clamp(value_+step_,min_,max_); return EventResult::Handled; } if(event.key==KeyCode::ArrowDown) { value_=std::clamp(value_-step_,min_,max_); return EventResult::Handled; } if(event.key==KeyCode::Escape) { focused_=false; return EventResult::Handled; } }
        return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; double value_, min_=-1e18, max_=1e18, step_=1.0; bool focused_=false;
};
} // namespace lumen::widgets
