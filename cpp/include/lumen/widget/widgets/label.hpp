#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Label : public Widget {
public:
    Label(std::string text) : text_(std::move(text)) { style_ = Style().text_sm().build(); }
    Label& with_style(ResolvedStyle s) { style_ = s; return *this; }
    const std::string& text() const { return text_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Label"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { if(style_.background.a > 0) ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_; std::string text_;
};
} // namespace lumen::widgets
