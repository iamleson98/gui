use lumen::prelude::*;
use lumen::style::{Style, Tw};
use lumen::widget::widgets::{Label, Scroll};
struct Stress { n: usize }
impl App for Stress {
    type State = Stress;
    type Message = ();
    fn init() -> Self::State { Stress { n: 10_000 } }
    fn view(state: &Self::State, ui: &mut Ui) {
        ui.push(Label::new(format!("Stress: {} items", state.n)).with_style(Style::new().text_lg().font_bold().px_4().py_2().build()));
        let style = Style::new().px_3().py_1().text_sm().build();
        let mut labels: Vec<Element> = Vec::with_capacity(state.n);
        for i in 0..state.n { let id = Id::new("root").derive_index(i); labels.push(Element::new(id, Label::new(format!("Item {i:05}")).with_style(style.clone()))); }
        ui.push(Scroll::new(labels));
    }
    fn update(_s: &mut Self::State, _m: Self::Message) {}
}
fn main() { lumen::platform::run::<Stress>(AppBuilder::new().title("Stress").size(800, 600)); }
