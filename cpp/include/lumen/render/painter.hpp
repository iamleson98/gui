#pragma once
#include "lumen/render/mesh.hpp"
#include "lumen/core/vec2.hpp"
#include <vector>
#include <optional>
#include <functional>
#include <string>
namespace lumen {
class Painter {
public:
    Painter() = default;
    void clear() { mesh_.clear(); clip_stack_.clear(); z_=0; offset_=Vec2::ZERO; }
    const Mesh& mesh() const { return mesh_; }
    void push_clip(Rect r);
    void pop_clip();
    std::optional<Rect> current_clip() const;
    void translate(Vec2 o) { offset_ = offset_ + o; }
    void fill_rect(Rect r, Color c);
    void fill_rounded_rect(Rect r, Color c, Corners rad);
    void stroke_rect(Rect r, Color c, float w);
    /// Draw a simple drop shadow: a semi-transparent dark rounded rect
    /// offset below and slightly larger than the given rect. Call before
    /// filling the actual rect so the shadow is underneath.
    void fill_shadow(Rect r, Corners rad, Vec2 offset, float blur, Color color);
    /// Fill an SVG path inside `dst_rect`. The path is interpreted in the
    /// coordinate space defined by `src_rect` (typically 24x24).
    void fill_svg(const std::string& d, Rect dst_rect, Rect src_rect, Color color);
    /// Push a glyph quad. `uv` is [atlas_x, atlas_y, glyph_w, glyph_h] in
    /// atlas pixels.
    void push_glyph(Rect r, const uint16_t uv[4], Color c);
private:
    Mesh mesh_; std::vector<Rect> clip_stack_; float z_=0; Vec2 offset_=Vec2::ZERO;
    void bump_z() { z_ += 1.0f/65536.0f; }
};
} // namespace lumen
