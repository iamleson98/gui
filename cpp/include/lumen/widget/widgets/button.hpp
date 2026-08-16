#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include "lumen/core/color.hpp"
#include <string>
#include <functional>
#include <cmath>

namespace lumen::widgets {

/// Emitted when a button is clicked.
struct ButtonClicked { Id id; };

/// The visual variant of a button. Synced with Rust.
enum class ButtonVariant { Primary, Secondary, Ghost };

/// A push button with a text label.
class Button : public Widget {
public:
    /// Create a primary button (solid indigo-600, white text).
    Button(std::string label) : Button(std::move(label), ButtonVariant::Primary) {}

    /// Create a primary button.
    static Button primary(std::string label) { return Button(std::move(label), ButtonVariant::Primary); }
    /// Create a secondary button (outlined).
    static Button secondary(std::string label) { return Button(std::move(label), ButtonVariant::Secondary); }
    /// Create a ghost button (transparent).
    static Button ghost(std::string label) { return Button(std::move(label), ButtonVariant::Ghost); }

    Button& on_click(std::function<void(Id)> f) { on_click_ = std::move(f); return *this; }
    const std::string& label() const { return label_; }
    ButtonVariant variant() const { return variant_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Button"; }

    void paint(PaintCtx& ctx, const Rect& rect) const override {
        auto [bg, border] = resolved_colors();
        // Drop shadow under primary buttons (not when pressed).
        if (variant_ == ButtonVariant::Primary && !pressed_) {
            ctx.painter.fill_shadow(rect, style_.border_radius, Vec2(0,1), 2.0f, Color(15,23,42,30));
        }
        ctx.painter.fill_rounded_rect(rect, bg, style_.border_radius);
        if (variant_ == ButtonVariant::Secondary) {
            ctx.painter.stroke_rect(rect, border, 1.0f);
        }
    }

    EventResult on_event(EventCtx& ctx, const Event& event) override {
        switch(event.kind) {
            case Event::Kind::PointerMove: if(!hovered_) { hovered_=true; ctx.state.request_redraw(); ctx.state.set_cursor(Cursor::Pointer); return EventResult::Handled; } return EventResult::Ignored;
            case Event::Kind::PointerLeave: if(hovered_||pressed_) { hovered_=pressed_=false; ctx.state.request_redraw(); return EventResult::Handled; } return EventResult::Ignored;
            case Event::Kind::PointerDown: pressed_=true; ctx.state.capture_pointer(ctx.current_id); return EventResult::Handled;
            case Event::Kind::PointerUp: if(pressed_) { pressed_=false; if(on_click_) on_click_(ctx.current_id); ctx.state.release_pointer(); return EventResult::HandledAndRedraw; } return EventResult::Ignored;
            default: return EventResult::Ignored;
        }
    }

private:
    Button(std::string label, ButtonVariant variant) : label_(std::move(label)), variant_(variant) {
        switch (variant) {
            case ButtonVariant::Primary:
                style_ = Style().px_6().py_3().rounded_lg().bg(Color::TW_INDIGO_600).text_white().text_base().font_medium().cursor_pointer().build();
                break;
            case ButtonVariant::Secondary:
                style_ = Style().px_6().py_3().rounded_lg().bg(Color::WHITE).border(1).border_color_(Color::TW_SLATE_300).text_color(Color::TW_SLATE_700).text_base().font_medium().cursor_pointer().build();
                break;
            case ButtonVariant::Ghost:
                style_ = Style().px_4().py_2().rounded_lg().text_color(Color::TW_SLATE_600).text_base().font_medium().cursor_pointer().build();
                break;
        }
    }

    std::pair<Color, Color> resolved_colors() const {
        switch (variant_) {
            case ButtonVariant::Primary:
                if (pressed_) return {Color::TW_INDIGO_700, Color::TRANSPARENT};
                if (hovered_) return {Color::TW_INDIGO_500, Color::TRANSPARENT};
                return {Color::TW_INDIGO_600, Color::TRANSPARENT};
            case ButtonVariant::Secondary: {
                Color bg = pressed_ ? Color::TW_SLATE_100 : (hovered_ ? Color::TW_SLATE_50 : Color::WHITE);
                Color border = hovered_ ? Color::TW_SLATE_400 : Color::TW_SLATE_300;
                return {bg, border};
            }
            case ButtonVariant::Ghost: {
                Color bg = pressed_ ? Color::TW_SLATE_100 : (hovered_ ? Color::TW_SLATE_50 : Color::TRANSPARENT);
                return {bg, Color::TRANSPARENT};
            }
        }
        return {style_.background, style_.border_color};
    }

    ResolvedStyle style_;
    std::string label_;
    ButtonVariant variant_;
    bool hovered_=false, pressed_=false;
    std::function<void(Id)> on_click_;
};

} // namespace lumen::widgets
