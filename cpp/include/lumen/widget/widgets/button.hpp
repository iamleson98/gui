#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

struct ButtonClicked { Id id; };
class Button : public Widget {
public:
    Button(std::string label) : label_(std::move(label)) { style_ = Style().px_4().py_2().rounded_md().bg_primary().text_white().font_medium().cursor_pointer().build(); }
    Button& on_click(std::function<void(Id)> f) { on_click_ = std::move(f); return *this; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Button"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto c = style_.background; if(pressed_) c=c.lerp(Color::BLACK,0.15f); else if(hovered_) c=c.lerp(Color::WHITE,0.10f); ctx.painter.fill_rounded_rect(rect, c, style_.border_radius); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        switch(event.kind) {
            case Event::Kind::PointerMove: if(!hovered_) { hovered_=true; ctx.state.request_redraw(); ctx.state.set_cursor(Cursor::Pointer); return EventResult::Handled; } return EventResult::Ignored;
            case Event::Kind::PointerLeave: if(hovered_||pressed_) { hovered_=pressed_=false; ctx.state.request_redraw(); return EventResult::Handled; } return EventResult::Ignored;
            case Event::Kind::PointerDown: pressed_=true; ctx.state.capture_pointer(ctx.current_id); return EventResult::Handled;
            case Event::Kind::PointerUp: if(pressed_) { pressed_=false; if(on_click_) on_click_(ctx.current_id); ctx.state.release_pointer(); return EventResult::HandledAndRedraw; } return EventResult::Ignored;
            default: return EventResult::Ignored;
        }
    }
private:
    ResolvedStyle style_; std::string label_; bool hovered_=false, pressed_=false; std::function<void(Id)> on_click_;
};
} // namespace lumen::widgets
