#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

struct ToggleChanged { bool on; };
class Toggle : public Widget {
public:
    Toggle(bool on=false) : on_(on) { style_ = Style().w(44).h(24).rounded_full().bg(Color(203,213,225)).cursor_pointer().build(); }
    bool is_on() const { return on_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Toggle"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto track = on_ ? Color::TW_EMERALD_500 : Color(203,213,225); ctx.painter.fill_rounded_rect(rect, track, style_.border_radius); float r=rect.height()*0.4f, travel=rect.width()-r*2-4; float x=rect.min.x+2+r+travel*(on_?1:0), y=rect.center().y; ctx.painter.fill_rounded_rect(Rect::from_xywh(x-r,y-r,r*2,r*2), Color::WHITE, Corners::all(r)); }
    EventResult on_event(EventCtx&, const Event& event) override { if(event.kind==Event::Kind::PointerUp) { on_=!on_; return EventResult::HandledAndRedraw; } if(event.kind==Event::Kind::PointerDown) return EventResult::Handled; return EventResult::Ignored; }
private:
    ResolvedStyle style_; bool on_;
};
} // namespace lumen::widgets
