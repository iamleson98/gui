#pragma once
#include "lumen/core/vec2.hpp"
#include "lumen/core/rect.hpp"
#include "lumen/core/id.hpp"
#include "lumen/style/style.hpp"
#include <vector>
namespace lumen {
struct Constraints { Vec2 min=Vec2::ZERO, max=Vec2(INFINITY,INFINITY);
    static Constraints tight(Vec2 s) { return {s,s}; }
    Constraints loose() const { return {Vec2::ZERO, max}; }
    Vec2 constrain(Vec2 s) const { return Vec2(std::clamp(s.x,min.x,max.x), std::clamp(s.y,min.y,max.y)); }
};
struct LayoutNode { Id id; const ResolvedStyle* style=nullptr; std::vector<LayoutNode> children; };
struct LayoutRect { Id id; Rect rect; std::vector<LayoutRect> children; };
LayoutRect arrange(const LayoutNode& node, Vec2 viewport);
} // namespace lumen
