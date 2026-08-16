#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Scroll : public Widget {
public:
    Scroll(std::vector<Element> children) : children_(std::move(children)) { style_ = Style().overflow_hidden().rounded_md().bg(Color::TRANSPARENT).build(); }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return children_; }
    std::vector<Element>& children_mut() override { return children_; }
    std::string debug_name() const override { return "Scroll"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { if(style_.background.a>0) ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.push_clip(rect); ctx.painter.translate(Vec2(-offset_.x, -offset_.y)); }
    EventResult on_event(EventCtx& ctx, const Event& event) override { if(event.kind==Event::Kind::Scroll) { offset_.y=std::max(0.0f, offset_.y+event.delta.y*20.0f); return EventResult::HandledAndRedraw; } return EventResult::Ignored; }
private:
    ResolvedStyle style_; std::vector<Element> children_; Vec2 offset_;
};
} // namespace lumen::widgets
