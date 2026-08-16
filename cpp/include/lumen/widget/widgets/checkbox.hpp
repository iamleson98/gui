#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

struct Toggled { bool checked; };
class Checkbox : public Widget {
public:
    Checkbox(bool checked=false) : checked_(checked) { style_ = Style().w(18).h(18).rounded_md().border(2).border_color_(Color(148,163,184)).bg_white().cursor_pointer().build(); }
    bool checked() const { return checked_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Checkbox"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto bg = checked_ ? Color::TW_INDIGO_500 : style_.background; ctx.painter.fill_rounded_rect(rect, bg, style_.border_radius); ctx.painter.stroke_rect(rect, style_.border_color, 2.0f); if(checked_) ctx.painter.fill_rect(rect.inset(4.0f), Color::WHITE); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { ctx.state.capture_pointer(ctx.current_id); return EventResult::Handled; }
        if(event.kind==Event::Kind::PointerUp) { checked_=!checked_; ctx.state.release_pointer(); return EventResult::HandledAndRedraw; }
        return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; bool checked_;
};
} // namespace lumen::widgets
