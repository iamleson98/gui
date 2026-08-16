#pragma once
#include "lumen/core/color.hpp"
#include "lumen/core/rect.hpp"
#include "lumen/style/style.hpp"
#include <vector>
#include <cstdint>
namespace lumen {
#pragma pack(push,1)
struct Vertex { float pos[2]; float color[4]; uint16_t uv[4]; float z; uint32_t kind; };
#pragma pack(pop)
static_assert(sizeof(Vertex)==40, "Vertex must be 40 bytes");
using Index = uint32_t;
struct Mesh {
    std::vector<Vertex> vertices; std::vector<Index> indices;
    void clear() { vertices.clear(); indices.clear(); }
    bool empty() const { return vertices.empty(); }
    size_t vertex_count() const { return vertices.size(); }
    size_t index_count() const { return indices.size(); }
    void add_rect(Rect r, Color color, float z);
    void add_glyph(Rect r, const uint16_t uv[4], Color color, float z);
    void add_rounded_rect(Rect r, Color color, Corners radius, float z);
};
} // namespace lumen
