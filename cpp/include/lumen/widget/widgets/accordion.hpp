#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct AccordionSection { std::string header; std::vector<Element> content; bool open=false; bool disabled=false; };
class Accordion : public Widget {
public:
    Accordion(std::vector<AccordionSection> sections) : sections_(std::move(sections)) { style_ = Style().flex_col().border(1).border_color_(Color(226,232,240)).rounded_md().build(); }
    bool is_open(size_t i) const { return i<sections_.size() && sections_[i].open; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Accordion"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, Color::WHITE, style_.border_radius); ctx.painter.stroke_rect(rect, style_.border_color, 1.0f); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { float y=ctx.current_rect.min.y, hh=36;
            for(size_t i=0; i<sections_.size(); ++i) { if(Rect::from_xywh(ctx.current_rect.min.x, y, ctx.current_rect.width(), hh).contains(event.pos)) { sections_[i].open=!sections_[i].open; ctx.state.request_layout(); ctx.state.request_redraw(); return EventResult::HandledAndRedraw; } y+=hh; if(sections_[i].open) y+=60; }
        } return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; std::vector<AccordionSection> sections_;
};
} // namespace lumen::widgets
