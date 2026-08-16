#pragma once
#include <cmath>
#include <algorithm>
namespace lumen {
struct Vec2 {
    float x=0, y=0;
    static const Vec2 ZERO;
    constexpr Vec2() = default;
    constexpr Vec2(float x_, float y_) : x(x_), y(y_) {}
    Vec2 min(const Vec2& o) const { return Vec2(std::min(x,o.x), std::min(y,o.y)); }
    Vec2 max(const Vec2& o) const { return Vec2(std::max(x,o.x), std::max(y,o.y)); }
    float dot(const Vec2& o) const { return x*o.x + y*o.y; }
    float length() const { return std::sqrt(dot(*this)); }
    Vec2 normalize() const { float l=length(); return l>0 ? Vec2(x/l,y/l) : *this; }
    Vec2 operator+(const Vec2& o) const { return Vec2(x+o.x, y+o.y); }
    Vec2 operator-(const Vec2& o) const { return Vec2(x-o.x, y-o.y); }
    Vec2 operator*(float s) const { return Vec2(x*s, y*s); }
    bool operator==(const Vec2& o) const { return x==o.x && y==o.y; }
};
inline const Vec2 Vec2::ZERO(0,0);
} // namespace lumen
