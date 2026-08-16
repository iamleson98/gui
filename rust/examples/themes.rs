//! Themes example — shows the same UI in light and dark themes.

use lumen::prelude::*;
use lumen::style::Theme;
use lumen::widget::widgets::{Button, Card, Label};

struct Themes;
impl App for Themes {
    type State = Themes;
    type Message = ();
    fn init() -> Self::State { Themes }
    fn view(_s: &Self::State, ui: &mut Ui) {
        ui.push(Label::heading("Theme Preview"));
        ui.push(Label::caption("Light and dark theme comparison."));

        let body = vec![
            Element::new(Id::new("t-label"), Label::subheading("Actions")),
            Element::new(Id::new("t-primary"), Button::new("Primary")),
            Element::new(Id::new("t-secondary"), Button::secondary("Secondary")),
            Element::new(Id::new("t-ghost"), Button::ghost("Ghost")),
        ];
        ui.push(Card::new(body));
    }
    fn update(_s: &mut Self::State, _m: Self::Message) {}
}
fn main() {
    lumen::platform::run::<Themes>(
        AppBuilder::new().title("Themes").size(640, 480).theme(Theme::light())
    );
}
