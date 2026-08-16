#pragma once
#include "lumen/render/mesh.hpp"
#include "lumen/core/vec2.hpp"
#include <vector>
#include <optional>
#include <functional>
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
private:
    Mesh mesh_; std::vector<Rect> clip_stack_; float z_=0; Vec2 offset_=Vec2::ZERO;
    void bump_z() { z_ += 1.0f/65536.0f; }
};
} // namespace lumen
