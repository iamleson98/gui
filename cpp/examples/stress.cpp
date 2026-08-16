#include "lumen/widget/widgets/widgets.hpp"
#include "lumen/platform/app.hpp"
#include <iostream>
using namespace lumen;
using namespace lumen::widgets;
struct StressApp : App {
    void init() override { std::cout << "lumen stress test\n"; }
    void view(Ui& ui) override {
        ui.push<Label>("Stress test").with_style(Style().text_lg().font_bold().px_4().py_2().build());
        std::vector<Element> labels;
        for(int i=0; i<1000; ++i) { labels.push_back(Element(Id::from_str("root").derive_index(i), std::make_shared<Label>("Item "+std::to_string(i)))); }
        ui.push<Scroll>(labels);
    }
};
int main() { StressApp app; return run(app, AppBuilder().title_("Stress").size_(800, 600)); }
