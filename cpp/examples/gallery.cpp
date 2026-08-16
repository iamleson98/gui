#include "lumen/widget/widgets/widgets.hpp"
#include "lumen/platform/app.hpp"
#include <iostream>
using namespace lumen;
using namespace lumen::widgets;
struct GalleryApp : App {
    void init() override { std::cout << "lumen gallery\n"; }
    void view(Ui& ui) override {
        ui.push<Label>("Widget Gallery").with_style(Style().text_xl().font_bold().px_4().py_2().build());
        ui.push<Button>("Click me");
        ui.push<Checkbox>(false);
        ui.push<Slider>(0.0f, 100.0f, 50.0f);
        ui.push<Progress>(0.6f);
        ui.push<Toggle>(false);
        ui.push<Badge>("New", Color::TW_INDIGO_500);
        ui.push<Avatar>("JD", 40.0f);
    }
};
int main() { GalleryApp app; return run(app, AppBuilder().title_("Gallery").size_(800, 600)); }
