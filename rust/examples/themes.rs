use lumen::prelude::*;
use lumen::style::{Style, Theme, Tw};
use lumen::widget::widgets::{Button, Label};
struct Themes;
impl App for Themes {
    type State = Themes;
    type Message = ();
    fn init() -> Self::State { Themes }
    fn view(_s: &Self::State, ui: &mut Ui) {
        ui.push(Label::new("Themes").with_style(Style::new().text_xl().font_bold().px_4().py_2().build()));
        ui.push(Button::new("Light"));
        ui.push(Button::new("Dark"));
        ui.push(Button::new("HC"));
    }
    fn update(_s: &mut Self::State, _m: Self::Message) {}
}
fn main() { lumen::platform::run::<Themes>(AppBuilder::new().title("Themes").size(640, 480).theme(Theme::light())); }
