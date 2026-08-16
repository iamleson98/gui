#pragma once
#include <cstdint>
#include <string>
#include <optional>
#include <cmath>
#include <array>
#include <algorithm>

namespace lumen {
struct Color {
    uint8_t r=0, g=0, b=0, a=255;
    static const Color BLACK, WHITE, RED, GREEN, BLUE, YELLOW, TRANSPARENT, GRAY;
    // Tailwind color constants — kept in sync with Rust core/color.rs
    static const Color TW_INDIGO_400, TW_INDIGO_500, TW_INDIGO_600, TW_INDIGO_700;
    static const Color TW_EMERALD_400, TW_EMERALD_500, TW_EMERALD_600;
    static const Color TW_ROSE_400, TW_ROSE_500, TW_ROSE_600;
    static const Color TW_AMBER_400, TW_AMBER_500, TW_AMBER_600;
    static const Color TW_SLATE_50, TW_SLATE_100, TW_SLATE_200, TW_SLATE_300;
    static const Color TW_SLATE_400, TW_SLATE_500, TW_SLATE_600, TW_SLATE_700, TW_SLATE_800, TW_SLATE_900;
    constexpr Color() = default;
    constexpr Color(uint8_t r_, uint8_t g_, uint8_t b_, uint8_t a_=255) : r(r_),g(g_),b(b_),a(a_) {}
    static constexpr Color rgb(uint8_t r, uint8_t g, uint8_t b) { return Color(r,g,b,255); }
    static constexpr Color rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) { return Color(r,g,b,a); }
    static std::optional<Color> from_hex(const std::string& hex);
    Color with_alpha(uint8_t a) const { return Color(r,g,b,a); }
    float alpha_f32() const { return a / 255.0f; }
    std::array<float,4> to_linear_premul() const {
        float af = alpha_f32();
        auto s2l = [](uint8_t c) -> float { float f=c/255.0f; return f<=0.04045f ? f/12.92f : std::pow((f+0.055f)/1.055f, 2.4f); };
        return {s2l(r)*af, s2l(g)*af, s2l(b)*af, af};
    }
    Color lerp(const Color& o, float t) const {
        t = std::clamp(t, 0.0f, 1.0f);
        auto l = [](uint8_t a, uint8_t b, float f) -> uint8_t { return (uint8_t)std::round(a+(b-a)*f); };
        return Color(l(r,o.r,t), l(g,o.g,t), l(b,o.b,t), l(a,o.a,t));
    }
    bool operator==(const Color& o) const { return r==o.r&&g==o.g&&b==o.b&&a==o.a; }
    bool operator!=(const Color& o) const { return !(*this==o); }
};
inline const Color Color::BLACK(0,0,0), Color::WHITE(255,255,255), Color::RED(255,0,0), Color::GREEN(0,200,0), Color::BLUE(0,90,220), Color::YELLOW(255,220,0), Color::TRANSPARENT(0,0,0,0), Color::GRAY(128,128,128);
inline const Color Color::TW_INDIGO_400(129,140,248), Color::TW_INDIGO_500(99,102,241), Color::TW_INDIGO_600(79,70,229), Color::TW_INDIGO_700(67,56,202);
inline const Color Color::TW_EMERALD_400(52,211,153), Color::TW_EMERALD_500(16,185,129), Color::TW_EMERALD_600(5,150,105);
inline const Color Color::TW_ROSE_400(251,113,133), Color::TW_ROSE_500(244,63,94), Color::TW_ROSE_600(225,29,72);
inline const Color Color::TW_AMBER_400(251,191,36), Color::TW_AMBER_500(245,158,11), Color::TW_AMBER_600(217,119,6);
inline const Color Color::TW_SLATE_50(248,250,252), Color::TW_SLATE_100(241,245,249), Color::TW_SLATE_200(226,232,240), Color::TW_SLATE_300(203,213,225);
inline const Color Color::TW_SLATE_400(148,163,184), Color::TW_SLATE_500(100,116,139), Color::TW_SLATE_600(71,85,105), Color::TW_SLATE_700(51,65,85), Color::TW_SLATE_800(30,41,59), Color::TW_SLATE_900(15,23,42);
} // namespace lumen
