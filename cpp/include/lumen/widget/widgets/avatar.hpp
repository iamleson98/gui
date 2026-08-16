#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

class Avatar : public Widget {
public:
    Avatar(std::string initials, float size) { bg_ = avatar_color(initials); style_ = Style().w(size).h(size).rounded_full().bg(bg_).build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Avatar"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, bg_, style_.border_radius); }
    static Color avatar_color(const std::string& s) { static Color p[]={Color::TW_INDIGO_500,Color::TW_EMERALD_500,Color::TW_ROSE_500,Color::TW_AMBER_500}; uint64_t h=0; for(char c:s) h=h*31+c; return p[h%4]; }
private:
    ResolvedStyle style_; Color bg_;
};
} // namespace lumen::widgets
