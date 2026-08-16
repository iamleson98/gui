#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Dialog : public Widget {
public:
    Dialog(std::vector<Element> content) : content_(std::move(content)) { style_ = Style().p_4().rounded_lg().bg_white().shadow_lg().max_w(480).build(); }
    bool is_open() const { return is_open_; }
    void open_dialog() { is_open_=true; }
    void close() { is_open_=false; }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return is_open_ ? content_ : empty_; }
    std::vector<Element>& children_mut() override { return is_open_ ? content_ : empty_mut_; }
    std::string debug_name() const override { return "Dialog"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { if(!is_open_) return; ctx.painter.fill_rect(rect, Color::rgba(0,0,0,120)); auto panel=Rect::from_xywh(rect.center().x-240, rect.center().y-100, 480, 200); ctx.painter.fill_rounded_rect(panel, style_.background, style_.border_radius); }
    EventResult on_event(EventCtx& ctx, const Event& event) override { if(!is_open_) return EventResult::Ignored; if(event.kind==Event::Kind::PointerDown) { is_open_=false; ctx.state.request_redraw(); return EventResult::Handled; } if(event.kind==Event::Kind::KeyDown && event.key==KeyCode::Escape) { is_open_=false; ctx.state.request_redraw(); return EventResult::Handled; } return EventResult::Ignored; }
private:
    static inline std::vector<Element> empty_, empty_mut_;
    ResolvedStyle style_; std::vector<Element> content_; bool is_open_=false;
};
} // namespace lumen::widgets
