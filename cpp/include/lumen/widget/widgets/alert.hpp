#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

enum class AlertKind { Info, Success, Warning, Error };
inline std::pair<Color,Color> alert_colors(AlertKind k) {
    switch(k) { case AlertKind::Info: return {Color(219,234,254), Color::TW_INDIGO_500}; case AlertKind::Success: return {Color(209,250,229), Color::TW_EMERALD_500};
        case AlertKind::Warning: return {Color(254,249,195), Color::TW_AMBER_500}; default: return {Color(254,226,226), Color::TW_ROSE_500}; }
}
class Alert : public Widget {
public:
    Alert(AlertKind kind, std::string, std::string) : kind_(kind) { auto [bg,_] = alert_colors(kind); style_ = Style().px_4().py_3().rounded_md().bg(bg).border(1).build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Alert"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto [bg,border] = alert_colors(kind_); ctx.painter.fill_rounded_rect(rect, bg, style_.border_radius); ctx.painter.stroke_rect(rect, border, 2.0f); }
private:
    ResolvedStyle style_; AlertKind kind_;
};
} // namespace lumen::widgets
