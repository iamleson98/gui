#include "lumen/render/svg.hpp"
#include <cmath>
#include <sstream>
#include <cctype>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

namespace lumen {

// --- Bezier / arc samplers ---

static void sample_cubic(std::vector<Vec2>& out, Vec2 p0, Vec2 p1, Vec2 p2, Vec2 p3) {
    const int N = 16;
    for (int i = 1; i <= N; ++i) {
        float t = (float)i / N;
        float u = 1.0f - t;
        Vec2 p = p0*(u*u*u) + p1*(3.0f*u*u*t) + p2*(3.0f*u*t*t) + p3*(t*t*t);
        out.push_back(p);
    }
}

static void sample_quadratic(std::vector<Vec2>& out, Vec2 p0, Vec2 p1, Vec2 p2) {
    const int N = 12;
    for (int i = 1; i <= N; ++i) {
        float t = (float)i / N;
        float u = 1.0f - t;
        Vec2 p = p0*(u*u) + p1*(2.0f*u*t) + p2*(t*t);
        out.push_back(p);
    }
}

static void sample_arc(std::vector<Vec2>& out, Vec2 start, float rx, float ry, float x_rot_deg, bool large, bool sweep, Vec2 end) {
    float phi = x_rot_deg * (float)M_PI / 180.0f;
    float cos_phi = std::cos(phi), sin_phi = std::sin(phi);
    float dx = (start.x - end.x) / 2.0f;
    float dy = (start.y - end.y) / 2.0f;
    float x1p = cos_phi*dx + sin_phi*dy;
    float y1p = -sin_phi*dx + cos_phi*dy;
    rx = std::fabs(rx); if (rx < 1e-6f) rx = 1e-6f;
    ry = std::fabs(ry); if (ry < 1e-6f) ry = 1e-6f;
    float lambda = (x1p*x1p)/(rx*rx) + (y1p*y1p)/(ry*ry);
    float s = lambda > 1.0f ? std::sqrt(lambda) : 1.0f;
    rx *= s; ry *= s;
    float num = rx*rx*ry*ry - rx*rx*y1p*y1p - ry*ry*x1p*x1p;
    float den = rx*rx*y1p*y1p + ry*ry*x1p*x1p;
    float factor = (num < 0 || den < 0) ? 0.0f : std::sqrt(num / std::max(den, 1e-12f));
    float sign = (large == sweep) ? -1.0f : 1.0f;
    float cxp = sign * factor * (rx * y1p / ry);
    float cyp = sign * factor * (-ry * x1p / rx);
    float cx = cos_phi*cxp - sin_phi*cyp + (start.x + end.x) / 2.0f;
    float cy = sin_phi*cxp + cos_phi*cyp + (start.y + end.y) / 2.0f;
    auto angle = [&](float px, float py) -> float {
        float vx = (px - cx) / rx;
        float vy = (py - cy) / ry;
        float a = std::acos(std::clamp(vx, -1.0f, 1.0f));
        if (vy < 0) a = -a;
        return a;
    };
    float theta1 = angle(start.x, start.y);
    float dtheta = angle(end.x, end.y) - theta1;
    if (sweep && dtheta < 0) dtheta += 2.0f * (float)M_PI;
    if (!sweep && dtheta > 0) dtheta -= 2.0f * (float)M_PI;
    const int N = 24;
    for (int i = 1; i <= N; ++i) {
        float t = theta1 + dtheta * (float)i / N;
        float px = cx + rx * (cos_phi*std::cos(t) - sin_phi*std::sin(t));
        float py = cy + ry * (sin_phi*std::cos(t) + cos_phi*std::sin(t));
        out.push_back(Vec2(px, py));
    }
}

// --- Parser ---

namespace {
class Parser {
public:
    Parser(const std::string& s) : ss_(s) {}
    bool next_cmd(char& cmd, bool& abs) {
        char c;
        while (ss_ >> c) {
            if (c == ',' || std::isspace((unsigned char)c)) continue;
            if (std::isalpha((unsigned char)c)) {
                cmd = std::toupper(c);
                abs = std::isupper(c);
                return true;
            }
            // Number started — put back, no more commands.
            ss_.putback(c);
            return false;
        }
        return false;
    }
    bool peek_number() {
        char c;
        while (ss_ >> c) {
            if (c == ',' || std::isspace((unsigned char)c)) continue;
            if (std::isdigit((unsigned char)c) || c == '-' || c == '+' || c == '.') {
                ss_.putback(c);
                return true;
            }
            ss_.putback(c);
            return false;
        }
        return false;
    }
    bool read_number(float& out) {
        // Skip whitespace.
        char c;
        while (ss_ >> c) {
            if (c == ',' || std::isspace((unsigned char)c)) continue;
            ss_.putback(c);
            break;
        }
        if (ss_ >> out) return true;
        return false;
    }
    Vec2 read_pair() {
        float x = 0, y = 0;
        read_number(x); read_number(y);
        return Vec2(x, y);
    }
    bool read_pair_or_break(Vec2& out) {
        if (!peek_number()) return false;
        out = read_pair();
        return true;
    }
    bool read_flag_or_break(float& out) {
        if (!peek_number()) return false;
        return read_number(out);
    }
private:
    std::istringstream ss_;
};
} // namespace

SvgPath SvgPath::parse(const std::string& d) {
    SvgPath path;
    Parser p(d);
    Vec2 cur = Vec2::ZERO, start = Vec2::ZERO;
    std::vector<Vec2> current;
    bool first_cmd = true;

    char cmd; bool abs;
    while (p.next_cmd(cmd, abs)) {
        switch (cmd) {
            case 'M': {
                if (!current.empty()) path.sub_paths_.push_back(std::move(current));
                Vec2 pt = p.read_pair();
                cur = (abs || first_cmd) ? pt : cur + pt;
                start = cur;
                current.push_back(cur);
                Vec2 pt2;
                while (p.read_pair_or_break(pt2)) {
                    cur = abs ? pt2 : cur + pt2;
                    current.push_back(cur);
                }
                break;
            }
            case 'L': {
                Vec2 pt = p.read_pair();
                cur = abs ? pt : cur + pt;
                current.push_back(cur);
                Vec2 pt2;
                while (p.read_pair_or_break(pt2)) {
                    cur = abs ? pt2 : cur + pt2;
                    current.push_back(cur);
                }
                break;
            }
            case 'H': {
                float x;
                while (p.read_number(x)) {
                    cur = abs ? Vec2(x, cur.y) : Vec2(cur.x + x, cur.y);
                    current.push_back(cur);
                }
                break;
            }
            case 'V': {
                float y;
                while (p.read_number(y)) {
                    cur = abs ? Vec2(cur.x, y) : Vec2(cur.x, cur.y + y);
                    current.push_back(cur);
                }
                break;
            }
            case 'C': {
                Vec2 c1, c2, end;
                while (true) {
                    if (!p.read_pair_or_break(c1)) break;
                    if (!p.read_pair_or_break(c2)) break;
                    if (!p.read_pair_or_break(end)) break;
                    Vec2 cc1 = abs ? c1 : cur + c1;
                    Vec2 cc2 = abs ? c2 : cur + c2;
                    Vec2 eend = abs ? end : cur + end;
                    sample_cubic(current, cur, cc1, cc2, eend);
                    cur = eend;
                }
                break;
            }
            case 'Q': {
                Vec2 c1, end;
                while (true) {
                    if (!p.read_pair_or_break(c1)) break;
                    if (!p.read_pair_or_break(end)) break;
                    Vec2 cc1 = abs ? c1 : cur + c1;
                    Vec2 eend = abs ? end : cur + end;
                    sample_quadratic(current, cur, cc1, eend);
                    cur = eend;
                }
                break;
            }
            case 'A': {
                Vec2 radii, end;
                float x_rot = 0, large_f = 0, sweep_f = 0;
                while (true) {
                    if (!p.read_pair_or_break(radii)) break;
                    if (!p.read_number(x_rot)) break;
                    if (!p.read_flag_or_break(large_f)) break;
                    if (!p.read_flag_or_break(sweep_f)) break;
                    if (!p.read_pair_or_break(end)) break;
                    Vec2 eend = abs ? end : cur + end;
                    sample_arc(current, cur, radii.x, radii.y, x_rot, large_f != 0, sweep_f != 0, eend);
                    cur = eend;
                }
                break;
            }
            case 'Z': {
                if (!current.empty()) {
                    current.push_back(start);
                    path.sub_paths_.push_back(std::move(current));
                }
                cur = start;
                break;
            }
            default: break;
        }
        first_cmd = false;
    }
    if (!current.empty()) path.sub_paths_.push_back(std::move(current));
    return path;
}

void SvgPath::tessellate(Mesh& mesh, Rect dst_rect, Rect src_rect, Color color, float z) const {
    auto col_arr = color.to_linear_premul();
    float sx = dst_rect.width() / std::max(src_rect.width(), 1e-6f);
    float sy = dst_rect.height() / std::max(src_rect.height(), 1e-6f);
    float ox = dst_rect.min.x - src_rect.min.x * sx;
    float oy = dst_rect.min.y - src_rect.min.y * sy;
    auto transform = [&](Vec2 p) { return Vec2(p.x*sx + ox, p.y*sy + oy); };

    for (const auto& sub : sub_paths_) {
        if (sub.size() < 3) continue;
        // Centroid.
        float cx = 0, cy = 0;
        for (const auto& pt : sub) { cx += pt.x; cy += pt.y; }
        cx /= sub.size(); cy /= sub.size();
        Vec2 center = transform(Vec2(cx, cy));
        uint32_t base = mesh.vertices.size();
        Vertex cv;
        cv.pos[0] = center.x; cv.pos[1] = center.y;
        cv.color[0]=col_arr[0]; cv.color[1]=col_arr[1]; cv.color[2]=col_arr[2]; cv.color[3]=col_arr[3];
        cv.uv[0]=0; cv.uv[1]=0; cv.uv[2]=0; cv.uv[3]=0;
        cv.z = z; cv.kind = 0;
        mesh.vertices.push_back(cv);
        for (const auto& pt : sub) {
            Vec2 tp = transform(pt);
            Vertex v;
            v.pos[0] = tp.x; v.pos[1] = tp.y;
            v.color[0]=col_arr[0]; v.color[1]=col_arr[1]; v.color[2]=col_arr[2]; v.color[3]=col_arr[3];
            v.uv[0]=0; v.uv[1]=0; v.uv[2]=0; v.uv[3]=0;
            v.z = z; v.kind = 0;
            mesh.vertices.push_back(v);
        }
        uint32_t n = (uint32_t)sub.size();
        for (uint32_t i = 0; i < n; ++i) {
            uint32_t a = base + 1 + i;
            uint32_t b = base + 1 + ((i + 1) % n);
            mesh.indices.push_back(base);
            mesh.indices.push_back(a);
            mesh.indices.push_back(b);
        }
    }
}

void fill_svg_path(Mesh& mesh, const std::string& d, Rect dst_rect, Rect src_rect, Color color, float z) {
    SvgPath::parse(d).tessellate(mesh, dst_rect, src_rect, color, z);
}

} // namespace lumen
