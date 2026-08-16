#include "lumen/render/mesh.hpp"
#include <cmath>
#include <algorithm>
namespace lumen {
static float s2l(uint8_t c) { float f=c/255.0f; return f<=0.04045f ? f/12.92f : std::pow((f+0.055f)/1.055f, 2.4f); }
static std::array<float,4> tlp(Color c) { float a=c.alpha_f32(); return {s2l(c.r)*a, s2l(c.g)*a, s2l(c.b)*a, a}; }
void Mesh::add_rect(Rect r, Color color, float z) { auto col=tlp(color); uint32_t i=vertices.size(); vertices.insert(vertices.end(), {{r.min.x,r.min.y,col,{0,0,0,0},z,0},{r.max.x,r.min.y,col,{0,0,0,0},z,0},{r.max.x,r.max.y,col,{0,0,0,0},z,0},{r.min.x,r.max.y,col,{0,0,0,0},z,0}}); indices.insert(indices.end(), {i,i+1,i+2,i,i+2,i+3}); }
void Mesh::add_glyph(Rect r, const uint16_t uv[4], Color color, float z) { auto col=tlp(color); uint32_t i=vertices.size(); vertices.insert(vertices.end(), {{r.min.x,r.min.y,col,{uv[0],uv[1],uv[2],uv[3]},z,1},{r.max.x,r.min.y,col,{uv[0],uv[1],uv[2],uv[3]},z,1},{r.max.x,r.max.y,col,{uv[0],uv[1],uv[2],uv[3]},z,1},{r.min.x,r.max.y,col,{uv[0],uv[1],uv[2],uv[3]},z,1}}); indices.insert(indices.end(), {i,i+1,i+2,i,i+2,i+3}); }
void Mesh::add_rounded_rect(Rect r, Color color, Corners rad, float z) { auto col=tlp(color); float tl=std::min({rad.top_left,r.width()*0.5f,r.height()*0.5f}); add_rect(Rect::from_xywh(r.min.x+tl,r.min.y,r.width()-tl,r.height()), color, z); add_rect(*this, Rect::from_xywh(r.min.x,r.min.y+tl,tl,r.height()-tl), color, z); (void)0;
    // Simplified: just draw center + sides
}
} // namespace lumen
