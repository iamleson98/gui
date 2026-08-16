use lumen::prelude::*;
use lumen::style::{Style, Tw};
use lumen::widget::widgets::{Button, Label};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

struct Counter { value: Arc<AtomicI32> }
impl App for Counter {
    type State = Counter;
    type Message = ();
    fn init() -> Self::State { Counter { value: Arc::new(AtomicI32::new(0)) } }
    fn view(state: &Self::State, ui: &mut Ui) {
        let v = state.value.clone();
        ui.push(Button::new("+").on_click(move |_| { v.fetch_add(1, Ordering::Relaxed); }));
        ui.push(Label::new(format!("Count: {}", state.value.load(Ordering::Relaxed)))
            .with_style(Style::new().text_2xl().font_bold().text_center().px_4().py_2().build()));
        let v = state.value.clone();
        ui.push(Button::new("-").on_click(move |_| { v.fetch_sub(1, Ordering::Relaxed); }));
    }
    fn update(_s: &mut Self::State, _m: Self::Message) {}
}
fn main() { lumen::platform::run::<Counter>(AppBuilder::new().title("Counter").size(480, 240)); }
