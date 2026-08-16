// Themes example — light/dark theme comparison.
#include "lumen/widget/widgets/widgets.hpp"
#include "lumen/platform/app.hpp"
using namespace lumen;
using namespace lumen::widgets;

struct ThemesApp : App {
    void view(Ui& ui) override {
        ui.push<Label>(Label::heading("Theme Preview"));
        ui.push<Label>(Label::caption("Light and dark theme comparison."));

        std::vector<Element> body;
        body.push_back(Element(Id::from_str("t-label"), std::make_shared<Label>(Label::subheading("Actions"))));
        body.push_back(Element(Id::from_str("t-primary"), std::make_shared<Button>(Button::primary("Primary"))));
        body.push_back(Element(Id::from_str("t-secondary"), std::make_shared<Button>(Button::secondary("Secondary"))));
        body.push_back(Element(Id::from_str("t-ghost"), std::make_shared<Button>(Button::ghost("Ghost"))));
        ui.push<Card>(std::move(body));
    }
};

int main() {
    ThemesApp app;
    return run(app, AppBuilder().title_("Themes").size_(640, 480).theme_(Theme::light()));
}
