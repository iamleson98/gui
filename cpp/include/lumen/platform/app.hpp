#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/render/painter.hpp"
#include <functional>
namespace lumen {
class App {
public:
    virtual ~App() = default;
    virtual void init() {}
    virtual void view(Ui& ui) = 0;
    virtual void update() {}
};
struct AppBuilder {
    std::string title="lumen app"; uint32_t width=1280, height=720; Theme theme=Theme::light(); bool vsync=true;
    AppBuilder& title_(const std::string& t) { title=t; return *this; }
    AppBuilder& size_(uint32_t w, uint32_t h) { width=w; height=h; return *this; }
    AppBuilder& theme_(const Theme& t) { theme=t; return *this; }
};
int run(App& app, const AppBuilder& builder={});
} // namespace lumen
