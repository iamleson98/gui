#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <atomic>
#include <cmath>

namespace lumen::widgets {

struct ButtonClicked { Id id; };
class Button : public Widget {
public:
    Button(std::string label) : label_(std::move(label)) { style_ = Style().px_4().py_2().rounded_md().bg_primary().text_white().font_medium().cursor_pointer().build(); }
    Button& on_click(std::function<void(Id)> f) { on_click_ = std::move(f); return *this; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Button"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto c = style_.background; if(pressed_) c=c.lerp(Color::BLACK,0.15f); else if(hovered_) c=c.lerp(Color::WHITE,0.10f); ctx.painter.fill_rounded_rect(rect, c, style_.border_radius); }
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
    ResolvedStyle style_; std::string label_; bool hovered_=false, pressed_=false; std::function<void(Id)> on_click_;
};

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

class Container : public Widget {
public:
    Container(ResolvedStyle s, std::vector<Element> children = {}) : style_(s), children_(std::move(children)) {}
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return children_; }
    std::vector<Element>& children_mut() override { return children_; }
    std::string debug_name() const override { return "Container"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { if(style_.background.a > 0) ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_; std::vector<Element> children_;
};

struct Toggled { bool checked; };
class Checkbox : public Widget {
public:
    Checkbox(bool checked=false) : checked_(checked) { style_ = Style().w(18).h(18).rounded_md().border(2).border_color_(Color(148,163,184)).bg_white().cursor_pointer().build(); }
    bool checked() const { return checked_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Checkbox"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto bg = checked_ ? Color::TW_INDIGO_500 : style_.background; ctx.painter.fill_rounded_rect(rect, bg, style_.border_radius); ctx.painter.stroke_rect(rect, style_.border_color, 2.0f); if(checked_) ctx.painter.fill_rect(rect.inset(4.0f), Color::WHITE); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { ctx.state.capture_pointer(ctx.current_id); return EventResult::Handled; }
        if(event.kind==Event::Kind::PointerUp) { checked_=!checked_; ctx.state.release_pointer(); return EventResult::HandledAndRedraw; }
        return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; bool checked_;
};

struct SliderChanged { float value; };
class Slider : public Widget {
public:
    Slider(float min, float max, float value) : min_(min), max_(max), value_(std::clamp(value,min,max)) { style_ = Style().h(8).w_full().rounded_md().bg(Color(226,232,240)).cursor_pointer().build(); }
    float value() const { return value_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Slider"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); float t=(value_-min_)/std::max(1e-6f, max_-min_); ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width()*t, rect.height()), Color::TW_INDIGO_500, style_.border_radius); }
    EventResult on_event(EventCtx& ctx, const Event& event) override {
        if(event.kind==Event::Kind::PointerDown) { dragging_=true; ctx.state.capture_pointer(ctx.current_id); float t=std::clamp((event.pos.x-ctx.current_rect.min.x)/ctx.current_rect.width(),0.0f,1.0f); value_=min_+t*(max_-min_); return EventResult::HandledAndRedraw; }
        if(event.kind==Event::Kind::PointerUp) { if(dragging_) { dragging_=false; ctx.state.release_pointer(); return EventResult::HandledAndRedraw; } }
        return EventResult::Ignored;
    }
private:
    ResolvedStyle style_; float min_, max_, value_; bool dragging_=false;
};

class Progress : public Widget {
public:
    Progress(float value) : value_(std::clamp(value,0.0f,1.0f)) { style_ = Style().h(8).w_full().rounded_md().bg(Color(226,232,240)).build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Progress"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.fill_rounded_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width()*value_, rect.height()), Color::TW_INDIGO_500, style_.border_radius); }
private:
    ResolvedStyle style_; float value_;
};

struct ToggleChanged { bool on; };
class Toggle : public Widget {
public:
    Toggle(bool on=false) : on_(on) { style_ = Style().w(44).h(24).rounded_full().bg(Color(203,213,225)).cursor_pointer().build(); }
    bool is_on() const { return on_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Toggle"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { auto track = on_ ? Color::TW_EMERALD_500 : Color(203,213,225); ctx.painter.fill_rounded_rect(rect, track, style_.border_radius); float r=rect.height()*0.4f, travel=rect.width()-r*2-4; float x=rect.min.x+2+r+travel*(on_?1:0), y=rect.center().y; ctx.painter.fill_rounded_rect(Rect::from_xywh(x-r,y-r,r*2,r*2), Color::WHITE, Corners::all(r)); }
    EventResult on_event(EventCtx&, const Event& event) override { if(event.kind==Event::Kind::PointerUp) { on_=!on_; return EventResult::HandledAndRedraw; } if(event.kind==Event::Kind::PointerDown) return EventResult::Handled; return EventResult::Ignored; }
private:
    ResolvedStyle style_; bool on_;
};

class Badge : public Widget {
public:
    Badge(std::string text, Color color) : color_(color) { style_ = Style().px_2().py_1().rounded_full().build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Badge"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, color_, style_.border_radius); }
private:
    ResolvedStyle style_; Color color_;
};

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

class Scroll : public Widget {
public:
    Scroll(std::vector<Element> children) : children_(std::move(children)) { style_ = Style().overflow_hidden().rounded_md().bg(Color::TRANSPARENT).build(); }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return children_; }
    std::vector<Element>& children_mut() override { return children_; }
    std::string debug_name() const override { return "Scroll"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { if(style_.background.a>0) ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.push_clip(rect); ctx.painter.translate(Vec2(-offset_.x, -offset_.y)); }
    EventResult on_event(EventCtx& ctx, const Event& event) override { if(event.kind==Event::Kind::Scroll) { offset_.y=(offset_.y+event.delta.y*20).max(0); return EventResult::HandledAndRedraw; } return EventResult::Ignored; }
private:
    ResolvedStyle style_; std::vector<Element> children_; Vec2 offset_;
};

class Card : public Widget {
public:
    Card(std::vector<Element> body) : body_(std::move(body)) { style_ = Style().p_4().rounded_lg().bg_white().shadow_md().flex_col().gap_2().build(); }
    const ResolvedStyle& style() const override { return style_; }
    const std::vector<Element>& children() const override { return body_; }
    std::vector<Element>& children_mut() override { return body_; }
    std::string debug_name() const override { return "Card"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_; std::vector<Element> body_;
};

class Canvas : public Widget {
public:
    Canvas() { style_ = Style().w_full().h_full().bg_white().build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Canvas"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); }
private:
    ResolvedStyle style_;
};

} // namespace lumen::widgets
