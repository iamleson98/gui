// Gallery example — showcases the redesigned widget styling.
#include "lumen/widget/widgets/widgets.hpp"
#include "lumen/platform/app.hpp"
#include <iostream>
using namespace lumen;
using namespace lumen::widgets;

struct GalleryApp : App {
    void init() override { std::cout << "lumen gallery\n"; }
    void view(Ui& ui) override {
        // Page header.
        ui.push<Label>(Label::heading("lumen Widget Gallery"));
        ui.push<Label>(Label::caption("A showcase of the redesigned widget styling."));

        // Buttons card.
        {
            std::vector<Element> body;
            body.push_back(Element(Id::from_str("btn-label"), std::make_shared<Label>(Label::subheading("Buttons"))));
            body.push_back(Element(Id::from_str("btn-primary"), std::make_shared<Button>(Button::primary("Primary Action"))));
            body.push_back(Element(Id::from_str("btn-secondary"), std::make_shared<Button>(Button::secondary("Secondary"))));
            body.push_back(Element(Id::from_str("btn-ghost"), std::make_shared<Button>(Button::ghost("Ghost"))));
            ui.push<Card>(std::move(body));
        }

        // Inputs card.
        {
            std::vector<Element> body;
            body.push_back(Element(Id::from_str("inputs-label"), std::make_shared<Label>(Label::subheading("Inputs"))));
            body.push_back(Element(Id::from_str("checkbox"), std::make_shared<Checkbox>(false)));
            body.push_back(Element(Id::from_str("slider"), std::make_shared<Slider>(0.0f, 100.0f, 50.0f)));
            body.push_back(Element(Id::from_str("progress"), std::make_shared<Progress>(0.6f)));
            body.push_back(Element(Id::from_str("toggle"), std::make_shared<Toggle>(false)));
            ui.push<Card>(std::move(body));
        }

        // Icons card.
        {
            std::vector<Element> body;
            body.push_back(Element(Id::from_str("icons-label"), std::make_shared<Label>(Label::subheading("Icons"))));
            body.push_back(Element(Id::from_str("icon-star"), std::make_shared<Icon>(IconKind::Star, 32.0f)));
            body.push_back(Element(Id::from_str("icon-heart"), std::make_shared<Icon>(IconKind::Heart, 32.0f)));
            body.push_back(Element(Id::from_str("icon-check"), std::make_shared<Icon>(IconKind::Check, 32.0f)));
            body.push_back(Element(Id::from_str("icon-search"), std::make_shared<Icon>(IconKind::Search, 32.0f)));
            body.push_back(Element(Id::from_str("icon-settings"), std::make_shared<Icon>(IconKind::Settings, 32.0f)));
            ui.push<Card>(std::move(body));
        }

        // Feedback card.
        {
            std::vector<Element> body;
            body.push_back(Element(Id::from_str("feedback-label"), std::make_shared<Label>(Label::subheading("Feedback"))));
            body.push_back(Element(Id::from_str("badge"), std::make_shared<Badge>("New", Color::TW_INDIGO_500)));
            body.push_back(Element(Id::from_str("avatar"), std::make_shared<Avatar>("JD", 40.0f)));
            ui.push<Card>(std::move(body));
        }
    }
};

int main() {
    GalleryApp app;
    return run(app, AppBuilder().title_("Gallery").size_(900, 1000));
}
