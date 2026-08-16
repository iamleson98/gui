#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct SelectOption { std::string label; bool disabled=false; SelectOption(std::string l) : label(std::move(l)) {} };
class Select : public Widget {
public:
    static Select single(std::vector<SelectOption> opts) { return Select(std::move(opts)); }
    static Select multi(std::vector<SelectOption> opts) { return Select(std::move(opts)); }
    const std::vector<size_t>& selected() const { return selected_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Select"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.stroke_rect(rect, style_.border_color, 1.0f); }
    EventResult on_event(EventCtx& ctx, const Event& event) override { if(event.kind==Event::Kind::PointerDown) { open_=!open_; ctx.state.request_redraw(); return EventResult::Handled; } return EventResult::Ignored; }
private:
    Select(std::vector<SelectOption> opts) : options_(std::move(opts)) { style_ = Style().px_3().py_2().rounded_md().bg_white().border(1).cursor_pointer().text_sm().build(); }
    ResolvedStyle style_; std::vector<SelectOption> options_; std::vector<size_t> selected_; bool open_=false;
};
} // namespace lumen::widgets
