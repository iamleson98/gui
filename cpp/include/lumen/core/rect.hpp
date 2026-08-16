#pragma once
#include "lumen/core/vec2.hpp"
#include <algorithm>
namespace lumen {
struct Rect {
    Vec2 min, max;
    static const Rect ZERO;
    constexpr Rect() = default;
    constexpr Rect(Vec2 a, Vec2 b) : min(a), max(b) {}
    static Rect from_xywh(float x, float y, float w, float h) { return Rect(Vec2(x,y), Vec2(x+w,y+h)); }
    float width() const { return max.x - min.x; }
    float height() const { return max.y - min.y; }
    Vec2 center() const { return Vec2((min.x+max.x)*0.5f, (min.y+max.y)*0.5f); }
    bool contains(Vec2 p) const { return p.x>=min.x && p.x<max.x && p.y>=min.y && p.y<max.y; }
    Rect translate(Vec2 by) const { return Rect(min+by, max+by); }
    Rect inset(float by) const { return from_xywh(min.x+by, min.y+by, std::max(0.0f,width()-2*by), std::max(0.0f,height()-2*by)); }
    Rect intersect(const Rect& o) const { Vec2 mn=min.max(o.min), mx=max.min(o.max); return (mn.x<=mx.x&&mn.y<=mx.y) ? Rect(mn,mx) : ZERO; }
    bool operator==(const Rect& o) const { return min==o.min && max==o.max; }
};
inline const Rect Rect::ZERO(Vec2::ZERO, Vec2::ZERO);
} // namespace lumen
