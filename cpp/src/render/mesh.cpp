#include "lumen/render/mesh.hpp"
#include <cmath>
namespace lumen {
static float s2l(uint8_t c) { float f = c / 255.0f; return f <= 0.04045f ? f / 12.92f : std::pow((f + 0.055f) / 1.055f, 2.4f); }
static std::array<float,4> tlp(Color c) { float a = c.alpha_f32(); return {s2l(c.r)*a, s2l(c.g)*a, s2l(c.b)*a, a}; }

static Vertex make_vertex(float x, float y, std::array<float,4> col, float z, uint32_t kind, uint16_t u0, uint16_t u1, uint16_t u2, uint16_t u3) {
    Vertex v;
    v.pos[0] = x; v.pos[1] = y;
    v.color[0] = col[0]; v.color[1] = col[1]; v.color[2] = col[2]; v.color[3] = col[3];
    v.uv[0] = u0; v.uv[1] = u1; v.uv[2] = u2; v.uv[3] = u3;
    v.z = z; v.kind = kind;
    return v;
}

void Mesh::add_rect(Rect r, Color color, float z) {
    auto col = tlp(color);
    uint32_t i = vertices.size();
    vertices.push_back(make_vertex(r.min.x, r.min.y, col, z, 0, 0,0,0,0));
    vertices.push_back(make_vertex(r.max.x, r.min.y, col, z, 0, 0,0,0,0));
    vertices.push_back(make_vertex(r.max.x, r.max.y, col, z, 0, 0,0,0,0));
    vertices.push_back(make_vertex(r.min.x, r.max.y, col, z, 0, 0,0,0,0));
    indices.push_back(i); indices.push_back(i+1); indices.push_back(i+2);
    indices.push_back(i); indices.push_back(i+2); indices.push_back(i+3);
}

/// Add a glyph quad. `uv` is [atlas_x, atlas_y, glyph_w, glyph_h] in atlas
/// pixels. Each of the 4 vertices gets its own UV so the quad samples the
/// correct sub-rectangle of the glyph atlas.
void Mesh::add_glyph(Rect r, const uint16_t uv[4], Color color, float z) {
    auto col = tlp(color);
    uint16_t ux = uv[0], uy = uv[1], uw = uv[2], uh = uv[3];
    uint32_t i = vertices.size();
    // top-left → (ux, uy)
    vertices.push_back(make_vertex(r.min.x, r.min.y, col, z, 1, ux, uy, 0, 0));
    // top-right → (ux + uw, uy)
    vertices.push_back(make_vertex(r.max.x, r.min.y, col, z, 1, ux+uw, uy, 0, 0));
    // bottom-right → (ux + uw, uy + uh)
    vertices.push_back(make_vertex(r.max.x, r.max.y, col, z, 1, ux+uw, uy+uh, 0, 0));
    // bottom-left → (ux, uy + uh)
    vertices.push_back(make_vertex(r.min.x, r.max.y, col, z, 1, ux, uy+uh, 0, 0));
    indices.push_back(i); indices.push_back(i+1); indices.push_back(i+2);
    indices.push_back(i); indices.push_back(i+2); indices.push_back(i+3);
}

/// Draw a rounded rect by decomposing it into:
/// - a central rect (full width minus the corner radii)
/// - two side rects (left and right strips between the corners)
/// - 4 quarter-circle corner fans (8 steps each)
void Mesh::add_rounded_rect(Rect r, Color color, Corners rad, float z) {
    auto col = tlp(color);
    float r_tl = std::min(rad.top_left, std::min(r.width()*0.5f, r.height()*0.5f));
    float r_tr = std::min(rad.top_right, std::min(r.width()*0.5f, r.height()*0.5f));
    float r_br = std::min(rad.bottom_right, std::min(r.width()*0.5f, r.height()*0.5f));
    float r_bl = std::min(rad.bottom_left, std::min(r.width()*0.5f, r.height()*0.5f));

    // Central rect (between the top and bottom corner radii, full width
    // minus the side radii).
    Rect center = Rect::from_xywh(r.min.x + r_tl, r.min.y, r.width() - r_tl - r_tr, r.height());
    add_rect(center, color, z);

    // Left strip (between the top-left and bottom-left corners).
    if (r_tl > 0 || r_bl > 0) {
        add_rect(Rect::from_xywh(r.min.x, r.min.y + r_tl, r_tl, r.height() - r_tl - r_bl), color, z);
    }
    // Right strip.
    if (r_tr > 0 || r_br > 0) {
        add_rect(Rect::from_xywh(r.max.x - r_tr, r.min.y + r_tr, r_tr, r.height() - r_tr - r_br), color, z);
    }

    // Four corner fans.
    const int STEPS = 8;
    struct Corner { float cx, cy, rad, a0, a1; };
    Corner corners[4] = {
        {r.min.x + r_tl, r.min.y + r_tl, r_tl, 180.0f, 270.0f},       // top-left
        {r.max.x - r_tr, r.min.y + r_tr, r_tr, 270.0f, 360.0f},       // top-right
        {r.max.x - r_br, r.max.y - r_br, r_br,   0.0f,  90.0f},       // bottom-right
        {r.min.x + r_bl, r.max.y - r_bl, r_bl,  90.0f, 180.0f},       // bottom-left
    };
    for (const auto& c : corners) {
        if (c.rad < 0.5f) continue;
        uint32_t base = vertices.size();
        // Center vertex.
        vertices.push_back(make_vertex(c.cx, c.cy, col, z, 0, 0,0,0,0));
        // Arc vertices.
        for (int i = 0; i <= STEPS; ++i) {
            float t = (float)i / STEPS;
            float a = (c.a0 + (c.a1 - c.a0) * t) * (float)M_PI / 180.0f;
            float x = c.cx + c.rad * std::cos(a);
            float y = c.cy + c.rad * std::sin(a);
            vertices.push_back(make_vertex(x, y, col, z, 0, 0,0,0,0));
        }
        for (int i = 0; i < STEPS; ++i) {
            indices.push_back(base);
            indices.push_back(base + 1 + i);
            indices.push_back(base + 2 + i);
        }
    }
}
} // namespace lumen
