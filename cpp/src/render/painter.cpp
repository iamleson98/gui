#include "lumen/render/painter.hpp"
#include "lumen/render/svg.hpp"
#include <algorithm>
namespace lumen {
void Painter::push_clip(Rect r) { r = r.translate(offset_); clip_stack_.push_back(clip_stack_.empty() ? r : clip_stack_.back().intersect(r)); }
void Painter::pop_clip() { if(!clip_stack_.empty()) clip_stack_.pop_back(); }
std::optional<Rect> Painter::current_clip() const { return clip_stack_.empty() ? std::nullopt : std::optional<Rect>(clip_stack_.back()); }
void Painter::fill_rect(Rect r, Color c) { r = r.translate(offset_); if(auto cl = current_clip()) r = r.intersect(*cl); if(r.width()<=0||r.height()<=0) return; mesh_.add_rect(r, c, z_); bump_z(); }
void Painter::fill_rounded_rect(Rect r, Color c, Corners rad) { r = r.translate(offset_); mesh_.add_rounded_rect(r, c, rad, z_); bump_z(); }
void Painter::stroke_rect(Rect r, Color c, float w) { r = r.translate(offset_); fill_rect(Rect::from_xywh(r.min.x,r.min.y,r.width(),w), c); fill_rect(Rect::from_xywh(r.min.x,r.max.y-w,r.width(),w), c); fill_rect(Rect::from_xywh(r.min.x,r.min.y+w*0.5f,w,r.height()-w), c); fill_rect(Rect::from_xywh(r.max.x-w,r.min.y+w*0.5f,w,r.height()-w), c); }
void Painter::fill_shadow(Rect r, Corners rad, Vec2 offset, float blur, Color color) {
    Rect shadow_rect = Rect::from_xywh(
        r.min.x + offset.x - blur,
        r.min.y + offset.y - blur,
        r.width() + blur * 2.0f,
        r.height() + blur * 2.0f
    );
    fill_rounded_rect(shadow_rect, color, rad);
}
void Painter::fill_svg(const std::string& d, Rect dst_rect, Rect src_rect, Color color) {
    dst_rect = dst_rect.translate(offset_);
    SvgPath path = SvgPath::parse(d);
    path.tessellate(mesh_, dst_rect, src_rect, color, z_);
    bump_z();
}
void Painter::push_glyph(Rect r, const uint16_t uv[4], Color c) {
    r = r.translate(offset_);
    mesh_.add_glyph(r, uv, c, z_);
    bump_z();
}
} // namespace lumen
