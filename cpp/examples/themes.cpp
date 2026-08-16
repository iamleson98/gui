#include "lumen/widget/widgets/widgets.hpp"
#include "lumen/platform/app.hpp"
using namespace lumen;
using namespace lumen::widgets;
struct ThemesApp : App {
    void view(Ui& ui) override {
        ui.push<Label>("Themes").with_style(Style().text_xl().font_bold().px_4().py_2().build());
        ui.push<Button>("Light"); ui.push<Button>("Dark"); ui.push<Button>("HC");
    }
};
int main() { ThemesApp app; return run(app, AppBuilder().title_("Themes").size_(640, 480)); }
