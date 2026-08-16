#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Chip : public Widget {
public:
    Chip(size_t id, std::string label) : id_(id), label_(std::move(label)) { style_ = Style().px_3().py_1().rounded_full().bg(Color(226,232,240)).text_sm().cursor_pointer().build(); }
    Chip& removable() { removable_=true; return *this; }
    Chip& selected() { selected_=true; return *this; }
    const std::string& label() const { return label_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Chip"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto bg=selected_?Color::TW_INDIGO_500:style_.background; ctx.painter.fill_rounded_rect(rect, bg, style_.border_radius); }
private:
    ResolvedStyle style_; size_t id_; std::string label_; bool removable_=false, selected_=false;
};
} // namespace lumen::widgets
