#include "lumen/render/software.hpp"
#include <cmath>
#include <algorithm>
#include <fstream>
#include <cstring>
#include <iostream>

namespace lumen {

// ---------------------------------------------------------------------------
// Color conversion helpers
// ---------------------------------------------------------------------------

static float l2s(float c) {
    c = std::clamp(c, 0.0f, 1.0f);
    return c <= 0.0031308f ? c * 12.92f : 1.055f * std::pow(c, 1.0f / 2.4f) - 0.055f;
}

static uint8_t f2u8(float v) {
    return (uint8_t)std::round(std::clamp(v, 0.0f, 1.0f) * 255.0f);
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

void SoftwareRenderer::init(const SoftwareRendererConfig& config) {
    width_ = config.width;
    height_ = config.height;
    output_path_ = config.output_path;
    framebuffer_.resize((size_t)width_ * height_ * 4, 0);
}

// ---------------------------------------------------------------------------
// Triangle rasterization (edge-function / barycentric)
// ---------------------------------------------------------------------------

static float edge(float ax, float ay, float bx, float by, float cx, float cy) {
    return (cx - ax) * (by - ay) - (cy - ay) * (bx - ax);
}

void SoftwareRenderer::blend_pixel(int x, int y, float r, float g, float b, float a) {
    if (x < 0 || y < 0 || (uint32_t)x >= width_ || (uint32_t)y >= height_) return;
    // a is in [0,1]; r,g,b are premultiplied-linear.
    size_t idx = ((size_t)y * width_ + x) * 4;
    // Framebuffer stores straight sRGB. Convert source to straight sRGB.
    float sa = a;
    if (sa <= 0.0f) return;
    // Un-premultiply.
    float sr = r / sa;
    float sg = g / sa;
    float sb = b / sa;
    // Linear to sRGB.
    sr = l2s(sr); sg = l2s(sg); sb = l2s(sb);
    // Blend source-over. Framebuffer is assumed opaque (background starts opaque).
    float dr = framebuffer_[idx]   / 255.0f;
    float dg = framebuffer_[idx+1] / 255.0f;
    float db = framebuffer_[idx+2] / 255.0f;
    float fr = sr * sa + dr * (1.0f - sa);
    float fg = sg * sa + dg * (1.0f - sa);
    float fb_ = sb * sa + db * (1.0f - sa);
    framebuffer_[idx]   = f2u8(fr);
    framebuffer_[idx+1] = f2u8(fg);
    framebuffer_[idx+2] = f2u8(fb_);
    framebuffer_[idx+3] = 255; // keep opaque
}

void SoftwareRenderer::rasterize_triangle(const Vertex& v0, const Vertex& v1, const Vertex& v2) {
    // Compute bounding box (clipped to screen).
    float minx = std::min({v0.pos[0], v1.pos[0], v2.pos[0]});
    float maxx = std::max({v0.pos[0], v1.pos[0], v2.pos[0]});
    float miny = std::min({v0.pos[1], v1.pos[1], v2.pos[1]});
    float maxy = std::max({v0.pos[1], v1.pos[1], v2.pos[1]});

    int x0 = std::max(0, (int)std::floor(minx));
    int x1 = std::min((int)width_ - 1, (int)std::ceil(maxx));
    int y0 = std::max(0, (int)std::floor(miny));
    int y1 = std::min((int)height_ - 1, (int)std::ceil(maxy));

    // Triangle area (edge function at v2).
    float area = edge(v0.pos[0], v0.pos[1], v1.pos[0], v1.pos[1], v2.pos[0], v2.pos[1]);
    if (std::fabs(area) < 1e-6f) return; // degenerate

    for (int y = y0; y <= y1; y++) {
        for (int x = x0; x <= x1; x++) {
            float px = x + 0.5f;
            float py = y + 0.5f;
            // Edge functions.
            float w0 = edge(v1.pos[0], v1.pos[1], v2.pos[0], v2.pos[1], px, py);
            float w1 = edge(v2.pos[0], v2.pos[1], v0.pos[0], v0.pos[1], px, py);
            float w2 = edge(v0.pos[0], v0.pos[1], v1.pos[0], v1.pos[1], px, py);
            // Check if inside (all same sign as area).
            bool inside = (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0);
            if (!inside) continue;
            // Barycentric coordinates.
            float b0 = w0 / area;
            float b1 = w1 / area;
            float b2 = w2 / area;
            // Interpolate premultiplied-linear color.
            float r = v0.color[0]*b0 + v1.color[0]*b1 + v2.color[0]*b2;
            float g = v0.color[1]*b0 + v1.color[1]*b1 + v2.color[1]*b2;
            float b = v0.color[2]*b0 + v1.color[2]*b1 + v2.color[2]*b2;
            float a = v0.color[3]*b0 + v1.color[3]*b1 + v2.color[3]*b2;
            blend_pixel(x, y, r, g, b, a);
        }
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

void SoftwareRenderer::render(const Mesh& mesh, Color clear_color) {
    // Fill background with clear color (straight sRGB, opaque).
    uint8_t cr = clear_color.r, cg = clear_color.g, cb = clear_color.b;
    for (size_t i = 0; i < framebuffer_.size(); i += 4) {
        framebuffer_[i] = cr;
        framebuffer_[i+1] = cg;
        framebuffer_[i+2] = cb;
        framebuffer_[i+3] = 255;
    }

    // Rasterize all triangles. The mesh indices are already sorted by z
    // (painter increments z for each draw call), so we just paint in order.
    for (size_t i = 0; i + 2 < mesh.indices.size(); i += 3) {
        uint32_t i0 = mesh.indices[i];
        uint32_t i1 = mesh.indices[i+1];
        uint32_t i2 = mesh.indices[i+2];
        if (i0 >= mesh.vertices.size() || i1 >= mesh.vertices.size() || i2 >= mesh.vertices.size())
            continue;
        const Vertex& v0 = mesh.vertices[i0];
        const Vertex& v1 = mesh.vertices[i1];
        const Vertex& v2 = mesh.vertices[i2];
        // Skip glyph (textured) triangles — no atlas in software mode.
        if (v0.kind == 1) continue;
        rasterize_triangle(v0, v1, v2);
    }

    // Save to PNG.
    if (!save_png(output_path_)) {
        std::cerr << "lumen: failed to save PNG to " << output_path_ << "\n";
    }
}

// ---------------------------------------------------------------------------
// Minimal PNG writer (uncompressed DEFLATE stored blocks)
// ---------------------------------------------------------------------------

static void write_u32_be(std::vector<uint8_t>& out, uint32_t val) {
    out.push_back((val >> 24) & 0xFF);
    out.push_back((val >> 16) & 0xFF);
    out.push_back((val >> 8) & 0xFF);
    out.push_back(val & 0xFF);
}

static uint32_t crc32_calc(const uint8_t* data, size_t len) {
    static uint32_t table[256];
    static bool init = false;
    if (!init) {
        for (uint32_t i = 0; i < 256; i++) {
            uint32_t c = i;
            for (int k = 0; k < 8; k++)
                c = (c & 1) ? (0xEDB88320 ^ (c >> 1)) : (c >> 1);
            table[i] = c;
        }
        init = true;
    }
    uint32_t crc = 0xFFFFFFFF;
    for (size_t i = 0; i < len; i++)
        crc = table[(crc ^ data[i]) & 0xFF] ^ (crc >> 8);
    return crc ^ 0xFFFFFFFF;
}

bool SoftwareRenderer::save_png(const std::string& path) const {
    // Build raw image data with per-scanline filter byte (0 = none).
    std::vector<uint8_t> raw;
    raw.reserve((size_t)(width_ * 4 + 1) * height_);
    for (uint32_t y = 0; y < height_; y++) {
        raw.push_back(0); // filter: none
        const uint8_t* row = &framebuffer_[(size_t)y * width_ * 4];
        raw.insert(raw.end(), row, row + width_ * 4);
    }

    // Compress with stored (uncompressed) DEFLATE blocks.
    std::vector<uint8_t> zlib;
    zlib.push_back(0x78); // CMF: deflate, window size 32K
    zlib.push_back(0x01); // FLG: no preset dict

    size_t offset = 0;
    while (offset < raw.size()) {
        size_t block_size = std::min(raw.size() - offset, (size_t)65535);
        bool last = (offset + block_size == raw.size());
        zlib.push_back(last ? 0x01 : 0x00); // BFINAL + BTYPE=00 (stored)
        uint16_t len = (uint16_t)block_size;
        zlib.push_back(len & 0xFF);
        zlib.push_back((len >> 8) & 0xFF);
        uint16_t nlen = ~len;
        zlib.push_back(nlen & 0xFF);
        zlib.push_back((nlen >> 8) & 0xFF);
        zlib.insert(zlib.end(), raw.begin() + offset, raw.begin() + offset + block_size);
        offset += block_size;
    }

    // Adler-32 checksum of the raw data.
    uint32_t s1 = 1, s2 = 0;
    for (size_t i = 0; i < raw.size(); i++) {
        s1 = (s1 + raw[i]) % 65521;
        s2 = (s2 + s1) % 65521;
    }
    uint32_t adler = (s2 << 16) | s1;
    write_u32_be(zlib, adler);

    // Assemble the PNG file.
    std::vector<uint8_t> png;
    // Signature.
    static const uint8_t sig[] = {137, 80, 78, 71, 13, 10, 26, 10};
    png.insert(png.end(), sig, sig + 8);

    // Helper to write a chunk.
    auto write_chunk = [&](const char* type, const uint8_t* data, size_t len) {
        std::vector<uint8_t> chunk;
        write_u32_be(chunk, (uint32_t)len);
        chunk.insert(chunk.end(), type, type + 4);
        chunk.insert(chunk.end(), data, data + len);
        uint32_t crc = crc32_calc(chunk.data() + 4, len + 4);
        write_u32_be(chunk, crc);
        png.insert(png.end(), chunk.begin(), chunk.end());
    };

    // IHDR.
    std::vector<uint8_t> ihdr;
    write_u32_be(ihdr, width_);
    write_u32_be(ihdr, height_);
    ihdr.push_back(8);  // bit depth
    ihdr.push_back(6);  // color type: RGBA
    ihdr.push_back(0);  // compression: deflate
    ihdr.push_back(0);  // filter: adaptive
    ihdr.push_back(0);  // interlace: none
    write_chunk("IHDR", ihdr.data(), ihdr.size());

    // IDAT.
    write_chunk("IDAT", zlib.data(), zlib.size());

    // IEND.
    write_chunk("IEND", nullptr, 0);

    // Write to file.
    std::ofstream file(path, std::ios::binary);
    if (!file) return false;
    file.write((const char*)png.data(), png.size());
    return true;
}

} // namespace lumen
