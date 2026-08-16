#include "lumen/layout/layout.hpp"
#include <algorithm>
#include <cmath>
namespace lumen {
static std::pair<Vec2, std::vector<LayoutRect>> measure_and_arrange(const LayoutNode& node, Constraints c);
static Vec2 apply_sizing(const ResolvedStyle& s, Vec2 intrinsic, Constraints c) { Vec2 sz=intrinsic; if(s.width && !std::isinf(*s.width)) sz.x=*s.width; if(s.height && !std::isinf(*s.height)) sz.y=*s.height; return c.constrain(sz); }

static std::pair<Vec2, std::vector<LayoutRect>> measure_and_arrange(const LayoutNode& node, Constraints c) {
    const ResolvedStyle& s = *node.style;
    float mw = s.margin.left+s.margin.right+s.border_width.left+s.border_width.right+s.padding.left+s.padding.right;
    float mh = s.margin.top+s.margin.bottom+s.border_width.top+s.border_width.bottom+s.padding.top+s.padding.bottom;
    Vec2 outer(mw, mh);
    Constraints inner_c = {Vec2(std::max(0.0f,c.min.x-outer.x), std::max(0.0f,c.min.y-outer.y)), Vec2(std::max(0.0f,c.max.x-outer.x), std::max(0.0f,c.max.y-outer.y))};
    if(node.children.empty()) { return {c.constrain(apply_sizing(s, Vec2::ZERO, inner_c)+outer), {}}; }
    bool horiz = s.flex_direction==FlexDirection::Row || s.flex_direction==FlexDirection::RowReverse;
    Constraints cc = horiz ? Constraints{Vec2::ZERO, Vec2(INFINITY, inner_c.max.y)} : Constraints{Vec2::ZERO, Vec2(inner_c.max.x, INFINITY)};
    struct E { float basis, cross; std::vector<LayoutRect> sub; };
    std::vector<E> entries; float total_basis=0, total_grow=0;
    for(auto& child : node.children) { auto [sz, sub] = measure_and_arrange(child, cc); float b=child.style->flex_basis.value_or(horiz?sz.x:sz.y); float cr=horiz?sz.y:sz.x; total_basis+=b; total_grow+=child.style->flex_grow; entries.push_back({b, cr, std::move(sub)}); }
    float main_avail = horiz ? inner_c.max.x : inner_c.max.y;
    float gap_total = entries.empty() ? 0 : s.gap*(entries.size()-1);
    float free = std::max(0.0f, main_avail - total_basis - gap_total);
    float gs = total_grow > 0 ? total_grow : 1;
    std::vector<float> fm(entries.size());
    for(size_t i=0; i<entries.size(); ++i) fm[i] = total_grow > 0 ? entries[i].basis + free*(node.children[i].style->flex_grow/gs) : entries[i].basis;
    float max_cross = 0; for(auto& e : entries) max_cross = std::max(max_cross, e.cross);
    Align align = s.align_items;
    std::vector<LayoutRect> layouts; float pos = 0;
    for(size_t i=0; i<entries.size(); ++i) {
        float main=fm[i], cross=entries[i].cross;
        float cp = (align==Align::Center) ? (max_cross-cross)*0.5f : (align==Align::End ? max_cross-cross : 0);
        float x = horiz ? pos : cp, y = horiz ? cp : pos;
        float w = horiz ? main : (align==Align::Stretch ? max_cross : cross);
        float h = horiz ? (align==Align::Stretch ? max_cross : cross) : main;
        for(auto& s2 : entries[i].sub) s2.rect = s2.rect.translate(Vec2(x, y));
        layouts.push_back({node.children[i].id, Rect::from_xywh(x,y,w,h), std::move(entries[i].sub)});
        pos += main + s.gap;
    }
    float im = 0; for(float v : fm) im += v; im += gap_total;
    Vec2 intrinsic = horiz ? Vec2(im, max_cross) : Vec2(max_cross, im);
    return {apply_sizing(s, intrinsic, inner_c) + outer, std::move(layouts)};
}

LayoutRect arrange(const LayoutNode& node, Vec2 viewport) {
    auto [size, children] = measure_and_arrange(node, Constraints::tight(viewport).loose());
    return {node.id, Rect::from_min_size(Vec2::ZERO, size), std::move(children)};
}
} // namespace lumen
