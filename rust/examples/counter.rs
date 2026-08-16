//! Counter example — a minimal app showing the redesigned button + label.

use lumen::prelude::*;
use lumen::style::{Style, Tw};
use lumen::widget::widgets::{Button, Card, Label};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

struct Counter {
    value: Arc<AtomicI32>,
}

impl App for Counter {
    type State = Counter;
    type Message = ();
    fn init() -> Self::State {
        Counter {
            value: Arc::new(AtomicI32::new(0)),
        }
    }

    fn view(state: &Self::State, ui: &mut Ui) {
        // Title
        ui.push(Label::heading("Counter"));

        // A card containing the counter value and +/- buttons.
        let v_dec = state.value.clone();
        let v_inc = state.value.clone();
        let body = vec![
            Element::new(Id::new("counter-label"), Label::heading(format!("{}", state.value.load(Ordering::Relaxed)))
                .with_style(Style::new().text_3xl().font_bold().text_center().build())),
            Element::new(Id::new("dec"), Button::secondary("-").on_click(move |_| {
                v_dec.fetch_sub(1, Ordering::Relaxed);
            })),
            Element::new(Id::new("inc"), Button::new("+").on_click(move |_| {
                v_inc.fetch_add(1, Ordering::Relaxed);
            })),
        ];
        ui.push(Card::new(body));
    }

    fn update(_s: &mut Self::State, _m: Self::Message) {}
}

fn main() {
    lumen::platform::run::<Counter>(
        AppBuilder::new().title("Counter").size(480, 360),
    );
}
