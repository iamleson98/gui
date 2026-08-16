#pragma once
#include "lumen/render/mesh.hpp"
#include "lumen/core/color.hpp"
#include <vector>
#include <cstdint>
#include <string>

namespace lumen {

/// Configuration for the software renderer.
struct SoftwareRendererConfig {
    uint32_t width = 1280;
    uint32_t height = 720;
    std::string output_path = "lumen_output.png";
};

/// A CPU-only triangle rasterizer that renders the mesh to an RGBA
/// framebuffer and saves it as a PNG file. Works on all platforms
/// (Linux, macOS, Windows) with no GPU or windowing system required.
///
/// This is the universal fallback when the GL backend is unavailable.
class SoftwareRenderer {
public:
    SoftwareRenderer() = default;

    /// Initialize with the given config (allocates the framebuffer).
    void init(const SoftwareRendererConfig& config);

    /// Render the mesh to the framebuffer, then save to the configured
    /// output path as a PNG file. `clear_color` is the background.
    void render(const Mesh& mesh, Color clear_color);

    /// Save the current framebuffer to a PNG file.
    bool save_png(const std::string& path) const;

    uint32_t width() const { return width_; }
    uint32_t height() const { return height_; }
    const std::vector<uint8_t>& framebuffer() const { return framebuffer_; }

private:
    uint32_t width_ = 1280;
    uint32_t height_ = 720;
    std::vector<uint8_t> framebuffer_; // RGBA, 4 bytes per pixel
    std::string output_path_;

    /// Rasterize a single triangle into the framebuffer.
    void rasterize_triangle(const Vertex& v0, const Vertex& v1, const Vertex& v2);

    /// Blend a premultiplied-linear RGBA source over a framebuffer pixel.
    void blend_pixel(int x, int y, float r, float g, float b, float a);
};

} // namespace lumen
