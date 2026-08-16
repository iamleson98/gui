use lumen::prelude::*;
use lumen::style::{Style, Tw};
use lumen::widget::widgets::*;
struct Gallery;
impl App for Gallery {
    type State = Gallery;
    type Message = ();
    fn init() -> Self::State { Gallery }
    fn view(_s: &Self::State, ui: &mut Ui) {
        ui.push(Label::new("Widget Gallery").with_style(Style::new().text_xl().font_bold().px_4().py_2().build()));
        ui.push(Button::new("Click me"));
        ui.push(Checkbox::new(false));
        ui.push(Slider::new(0.0, 100.0, 50.0));
        ui.push(Progress::new(0.6));
        ui.push(Toggle::new(false));
        ui.push(Badge::new("New", Color::TW_INDIGO_500));
        ui.push(Avatar::new("JD", 40.0));
    }
    fn update(_s: &mut Self::State, _m: Self::Message) {}
}
fn main() { lumen::platform::run::<Gallery>(AppBuilder::new().title("Gallery").size(800, 600)); }
