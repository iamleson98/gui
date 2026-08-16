#include "lumen/render/mesh.hpp"
#include <cmath>
namespace lumen {
static float s2l(uint8_t c) { float f = c / 255.0f; return f <= 0.04045f ? f / 12.92f : std::pow((f + 0.055f) / 1.055f, 2.4f); }
static std::array<float,4> tlp(Color c) { float a = c.alpha_f32(); return {s2l(c.r)*a, s2l(c.g)*a, s2l(c.b)*a, a}; }

static Vertex make_vertex(float x, float y, std::array<float,4> col, float z, uint32_t kind, const uint16_t uv[4]) {
    Vertex v;
    v.pos[0] = x; v.pos[1] = y;
    v.color[0] = col[0]; v.color[1] = col[1]; v.color[2] = col[2]; v.color[3] = col[3];
    v.uv[0] = uv[0]; v.uv[1] = uv[1]; v.uv[2] = uv[2]; v.uv[3] = uv[3];
    v.z = z; v.kind = kind;
    return v;
}

void Mesh::add_rect(Rect r, Color color, float z) {
    auto col = tlp(color);
    uint16_t uv[4] = {0,0,0,0};
    uint32_t i = vertices.size();
    vertices.push_back(make_vertex(r.min.x, r.min.y, col, z, 0, uv));
    vertices.push_back(make_vertex(r.max.x, r.min.y, col, z, 0, uv));
    vertices.push_back(make_vertex(r.max.x, r.max.y, col, z, 0, uv));
    vertices.push_back(make_vertex(r.min.x, r.max.y, col, z, 0, uv));
    indices.push_back(i); indices.push_back(i+1); indices.push_back(i+2);
    indices.push_back(i); indices.push_back(i+2); indices.push_back(i+3);
}

void Mesh::add_glyph(Rect r, const uint16_t uv[4], Color color, float z) {
    auto col = tlp(color);
    uint32_t i = vertices.size();
    vertices.push_back(make_vertex(r.min.x, r.min.y, col, z, 1, uv));
    vertices.push_back(make_vertex(r.max.x, r.min.y, col, z, 1, uv));
    vertices.push_back(make_vertex(r.max.x, r.max.y, col, z, 1, uv));
    vertices.push_back(make_vertex(r.min.x, r.max.y, col, z, 1, uv));
    indices.push_back(i); indices.push_back(i+1); indices.push_back(i+2);
    indices.push_back(i); indices.push_back(i+2); indices.push_back(i+3);
}

void Mesh::add_rounded_rect(Rect r, Color color, Corners rad, float z) {
    add_rect(r, color, z);
}
} // namespace lumen
