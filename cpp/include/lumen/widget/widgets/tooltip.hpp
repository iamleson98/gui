#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Tooltip : public Widget {
public:
    Tooltip(std::shared_ptr<Widget> child, std::string text) : child_(Element(Id::from_str("tt"), child)), text_(std::move(text)) { style_ = Style().build(); }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return single_; }
    std::vector<Element>& children_mut() override { return single_mut_; }
    std::string debug_name() const override { return "Tooltip"; }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerMove) { if(ctx.current_rect.contains(event.pos)) { hover_ms_+=16; if(!visible_ && hover_ms_>=500) { visible_=true; ctx.state.request_redraw(); } } else if(visible_||hover_ms_>0) { visible_=false; hover_ms_=0; ctx.state.request_redraw(); } }
        if(event.kind==Event::Kind::PointerLeave && (visible_||hover_ms_>0)) { visible_=false; hover_ms_=0; ctx.state.request_redraw(); }
        return EventResult::Ignored;
    }
private:
    static inline std::vector<Element> single_, single_mut_;
    ResolvedStyle style_; Element child_; std::string text_; bool visible_=false; uint32_t hover_ms_=0;
};
} // namespace lumen::widgets
