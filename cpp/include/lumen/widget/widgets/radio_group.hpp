#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class RadioGroup : public Widget {
public:
    RadioGroup(std::vector<std::string> options) : options_(std::move(options)) { style_ = Style().flex_col().gap_2().build(); }
    RadioGroup& with_selected(size_t idx) { if(idx < options_.size()) selected_ = idx; return *this; }
    std::optional<size_t> selected() const { return selected_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "RadioGroup"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        float n = options_.size(); if(n==0) return; float rh = rect.height()/n;
        for(size_t i=0; i<n; ++i) { float y=rect.min.y+rh*i; bool sel=selected_==i; float cx=rect.min.x+8, cy=y+rh*0.5, r=8;
            ctx.painter.fill_rounded_rect(Rect::from_xywh(cx-r,cy-r,r*2,r*2), Color::WHITE, Corners::all(r));
            ctx.painter.stroke_rect(Rect::from_xywh(cx-r,cy-r,r*2,r*2), sel?Color::TW_INDIGO_500:Color(203,213,225), 2.0f);
            if(sel) ctx.painter.fill_rounded_rect(Rect::from_xywh(cx-4,cy-4,8,8), Color::TW_INDIGO_500, Corners::all(4));
        }
    }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { size_t n=options_.size(); if(n==0) return EventResult::Ignored;
            float rh=ctx.current_rect.height()/n; float ry=event.pos.y-ctx.current_rect.min.y;
            if(ry>=0 && ry<=ctx.current_rect.height()) { size_t idx=ry/rh; if(idx<n) { selected_=idx; ctx.state.request_redraw(); return EventResult::HandledAndRedraw; } }
        } return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; std::vector<std::string> options_; std::optional<size_t> selected_;
};
} // namespace lumen::widgets
