#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Progress : public Widget {
public:
    Progress(float value) : value_(std::clamp(value,0.0f,1.0f)) { style_ = Style().h(8).w_full().rounded_md().bg(Color(226,232,240)).build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Progress"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width()*value_, rect.height()), Color::TW_INDIGO_500, style_.border_radius); }
private:
    ResolvedStyle style_; float value_;
};
} // namespace lumen::widgets
