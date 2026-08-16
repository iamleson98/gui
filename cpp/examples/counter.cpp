#include "lumen/widget/widgets/widgets.hpp"
#include "lumen/platform/app.hpp"
#include <atomic>
#include <iostream>
using namespace lumen;
using namespace lumen::widgets;

struct CounterApp : App {
    std::atomic<int> value{0};
    void init() override { std::cout << "lumen counter\n"; }
    void view(Ui& ui) override {
        auto v = value.load(); auto& p = ui.push<Label>("+"); (void)p;
        ui.push<Button>("+").on_click([this](Id){ value.fetch_add(1); });
        ui.push<Label>(std::string("Count: ") + std::to_string(value.load())).with_style(Style().text_2xl().font_bold().text_center().px_4().py_2().build());
        ui.push<Button>("-").on_click([this](Id){ value.fetch_sub(1); });
    }
};
int main() { CounterApp app; return run(app, AppBuilder().title_("Counter").size_(480, 240)); }
