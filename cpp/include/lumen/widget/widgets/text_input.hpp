#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class TextInput : public Widget {
public:
    TextInput(std::string placeholder) : placeholder_(std::move(placeholder)) { style_ = Style().px_3().py_2().rounded_md().bg_white().border(1).border_color_(Color(203,213,225)).text_sm().build(); }
    TextInput& with_style(ResolvedStyle s) { style_ = s; return *this; }
    const std::string& text() const { return text_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "TextInput"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); auto border = focused_ ? Color::TW_INDIGO_500 : style_.border_color; ctx.painter.stroke_rect(rect, border, 1.0f); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { focused_=true; ctx.state.request_focus(ctx.current_id); return EventResult::Handled; }
        if(event.kind==Event::Kind::FocusLost && focused_) { focused_=false; return EventResult::Handled; }
        if(event.kind==Event::Kind::Char && focused_ && !std::iscntrl((unsigned char)event.ch)) { text_+=event.ch; return EventResult::Handled; }
        if(event.kind==Event::Kind::KeyDown && focused_) { if(event.key==KeyCode::Backspace && !text_.empty()) { text_.pop_back(); return EventResult::Handled; } if(event.key==KeyCode::Escape) { focused_=false; return EventResult::Handled; } }
        return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; std::string text_, placeholder_; bool focused_=false;
};
} // namespace lumen::widgets
