#pragma once
#include "lumen/render/mesh.hpp"
#include "lumen/core/rect.hpp"
#include "lumen/core/vec2.hpp"
#include "lumen/core/color.hpp"
#include <string>
#include <vector>
namespace lumen {

/// A parsed SVG path, ready to tessellate into the mesh.
class SvgPath {
public:
    /// Parse SVG path data. Supports the standard `d="..."` string format:
    /// M/m, L/l, H/h, V/v, C/c, Q/q, A/a, Z/z (absolute and relative).
    static SvgPath parse(const std::string& d);

    /// Tessellate the path into the mesh as filled triangles (triangle fan
    /// from each sub-path's centroid). The path is mapped from its source
    /// viewBox (`src_rect`) into `dst_rect`.
    void tessellate(Mesh& mesh, Rect dst_rect, Rect src_rect, Color color, float z) const;

    const std::vector<std::vector<Vec2>>& sub_paths() const { return sub_paths_; }

private:
    std::vector<std::vector<Vec2>> sub_paths_;
};

/// Convenience: parse + tessellate a single SVG path string.
void fill_svg_path(Mesh& mesh, const std::string& d, Rect dst_rect, Rect src_rect, Color color, float z);

} // namespace lumen
