// Counter example — minimal app showing the redesigned button + card.
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
        ui.push<Label>(Label::heading("Counter"));

        std::vector<Element> body;
        body.push_back(Element(Id::from_str("counter-label"),
            std::make_shared<Label>(Label::heading(std::to_string(value.load())))));
        body.push_back(Element(Id::from_str("dec"),
            std::make_shared<Button>(Button::secondary("-"))));
        body.push_back(Element(Id::from_str("inc"),
            std::make_shared<Button>(Button::primary("+"))));
        ui.push<Card>(std::move(body));
    }
};

int main() {
    CounterApp app;
    return run(app, AppBuilder().title_("Counter").size_(480, 360));
}
