#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/core/color.hpp"
#include <string>

namespace lumen::widgets {

/// The visual variant of a label. Synced with Rust.
enum class LabelVariant { Heading, Subheading, Body, Caption };

/// A non-interactive text label.
class Label : public Widget {
public:
    /// Create a default body-text label (14px, slate-900).
    Label(std::string text) : Label(std::move(text), LabelVariant::Body) {}

    /// Create a large heading label (28px, bold).
    static Label heading(std::string text) { return Label(std::move(text), LabelVariant::Heading); }
    /// Create a medium subheading label (18px, bold).
    static Label subheading(std::string text) { return Label(std::move(text), LabelVariant::Subheading); }
    /// Create a small muted caption label (12px).
    static Label caption(std::string text) { return Label(std::move(text), LabelVariant::Caption); }

    Label& with_style(ResolvedStyle s) { style_ = s; return *this; }
    const std::string& text() const { return text_; }
    LabelVariant variant() const { return variant_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Label"; }

    void paint(PaintCtx& ctx, const Rect& rect) const override {
        if (style_.background.a > 0) {
            ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius);
        }
        // C++ port does not yet have a text engine; the Rust side renders
        // actual glyphs via cosmic-text. Here we draw nothing for the text
        // itself, but the background (if any) is still rendered so cards
        // and containers look correct.
    }

private:
    Label(std::string text, LabelVariant variant) : text_(std::move(text)), variant_(variant) {
        switch (variant) {
            case LabelVariant::Heading:
                style_ = Style().text_2xl().font_bold().text_color(Color::TW_SLATE_900).build();
                break;
            case LabelVariant::Subheading:
                style_ = Style().text_lg().font_bold().text_color(Color::TW_SLATE_700).build();
                break;
            case LabelVariant::Body:
                style_ = Style().text_base().text_color(Color::TW_SLATE_900).build();
                break;
            case LabelVariant::Caption:
                style_ = Style().text_sm().text_color(Color::TW_SLATE_500).build();
                break;
        }
    }

    ResolvedStyle style_;
    std::string text_;
    LabelVariant variant_;
};

} // namespace lumen::widgets
