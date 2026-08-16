#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include "lumen/core/color.hpp"
#include "lumen/core/vec2.hpp"
#include <vector>

namespace lumen::widgets {

/// A surface container with a subtle border, drop shadow, and rounded
/// corners. Use to group related content into a visual section.
class Card : public Widget {
public:
    /// Create a card with the given children. White background, 1px
    /// slate-200 border, rounded-xl (14px), 24px padding, flex column.
    Card(std::vector<Element> body) : body_(std::move(body)) {
        style_ = Style()
            .p_6()
            .rounded_xl()
            .bg(Color::WHITE)
            .border(1)
            .border_color_(Color::TW_SLATE_200)
            .flex_col()
            .gap_4()
            .items_stretch()
            .build();
    }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return body_; }
    std::vector<Element>& children_mut() override { return body_; }
    std::string debug_name() const override { return "Card"; }

    void paint(PaintCtx& ctx, const Rect& rect) const override {
        // Subtle drop shadow underneath the card.
        ctx.painter.fill_shadow(rect, style_.border_radius, Vec2(0, 2), 4.0f, Color(15, 23, 42, 24));
        // Card background.
        ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius);
        // 1px border.
        if (style_.border_width.left > 0) {
            ctx.painter.stroke_rect(rect, style_.border_color, style_.border_width.left);
        }
    }

private:
    ResolvedStyle style_;
    std::vector<Element> body_;
};

} // namespace lumen::widgets
