#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Tabs : public Widget {
public:
    Tabs(std::vector<Element> tabs, size_t active) : tabs_(std::move(tabs)) { style_ = Style().flex_col().build(); active_ = std::min(active, tabs_.size()>0?tabs_.size()-1:0); }
    size_t active() const { return active_; }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return tabs_; }
    std::vector<Element>& children_mut() override { return tabs_; }
    std::string debug_name() const override { return "Tabs"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        ctx.painter.fill_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width(), 36), Color(241,245,249));
        size_t n=tabs_.size(); if(n==0) return; float tw=rect.width()/n;
        for(size_t i=0; i<n; ++i) { if(i==active_) { ctx.painter.fill_rect(Rect::from_xywh(rect.min.x+i*tw, rect.min.y, tw, 36), Color::WHITE);
            ctx.painter.fill_rect(Rect::from_xywh(rect.min.x+i*tw, rect.min.y+34, tw, 2), Color::TW_INDIGO_500); } }
    }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown && event.pos.y < ctx.current_rect.min.y+36) {
            size_t n=tabs_.size(); if(n==0) return EventResult::Ignored; float tw=ctx.current_rect.width()/n;
            size_t idx=(event.pos.x-ctx.current_rect.min.x)/tw; if(idx<n && idx!=active_) { active_=idx; ctx.state.request_redraw(); return EventResult::HandledAndRedraw; }
        } return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; std::vector<Element> tabs_; size_t active_;
};
} // namespace lumen::widgets
