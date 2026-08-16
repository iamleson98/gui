#pragma once
#include "lumen/render/painter.hpp"
#include <string>
#include <functional>
namespace lumen {
struct GLWindowConfig { std::string title="lumen"; uint32_t width=1280, height=720; bool vsync=true; };
class GLWindow {
public:
    GLWindow() = default;
    ~GLWindow();
    bool init(const GLWindowConfig& config);
    void run(std::function<void(Painter&)> on_frame, std::function<void(uint32_t,uint32_t)> on_resize={});
    uint32_t width() const { return width_; }
    uint32_t height() const { return height_; }
    void close() { should_close_ = true; }
private:
    void* display_=nullptr; void* window_=nullptr; void* gl_context_=nullptr;
    uint32_t width_=1280, height_=720; bool should_close_=false;
    uint32_t shader_program_=0, vao_=0, vbo_=0, ibo_=0;
    void cleanup();
};
} // namespace lumen
