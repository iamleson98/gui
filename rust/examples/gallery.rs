//! Gallery example — demonstrates the redesigned widget styling.
//!
//! Run with: `cargo run --example gallery`

use lumen::prelude::*;
use lumen::style::{Style, Theme, Tw};
use lumen::widget::widgets::{
    Alert, AlertKind, Badge, Card, Checkbox, Crumb, Breadcrumb, Button,
    ButtonVariant, Gauge, Icon, IconKind, Label, LabelVariant, NumberInput, Pagination,
    Progress, RangeSlider, Rating, Slider, Spinner, Stepper, Step, TextInput, Toggle,
};

struct Gallery;

impl App for Gallery {
    type State = Gallery;
    type Message = ();
    fn init() -> Self::State {
        Gallery
    }

    fn view(_s: &Self::State, ui: &mut Ui) {
        // ---- Page header ----
        ui.push(Label::heading("lumen Widget Gallery"));
        ui.push(
            Label::caption("A showcase of the redesigned widget styling.")
                .with_style(Style::new().text_sm().text_muted().build()),
        );

        // ---- Buttons card ----
        let buttons = vec![
            Element::new(
                Id::new("btn-card-label"),
                Label::subheading("Buttons"),
            ),
            Element::new(
                Id::new("btn-primary"),
                Button::new("Primary Action"),
            ),
            Element::new(
                Id::new("btn-secondary"),
                Button::secondary("Secondary"),
            ),
            Element::new(
                Id::new("btn-ghost"),
                Button::ghost("Ghost"),
            ),
        ];
        ui.push(Card::new(buttons));

        // ---- Inputs card ----
        let inputs = vec![
            Element::new(
                Id::new("inputs-label"),
                Label::subheading("Inputs"),
            ),
            Element::new(
                Id::new("text-input"),
                TextInput::new("Type something..."),
            ),
            Element::new(
                Id::new("checkbox"),
                Checkbox::new(false),
            ),
            Element::new(
                Id::new("toggle"),
                Toggle::new(true),
            ),
            Element::new(
                Id::new("slider"),
                Slider::new(0.0, 100.0, 50.0),
            ),
            Element::new(
                Id::new("range-slider"),
                RangeSlider::new(0.0, 100.0, 25.0, 75.0),
            ),
            Element::new(
                Id::new("number-input"),
                NumberInput::new(42.0).with_range(0.0, 100.0).with_step(1.0),
            ),
        ];
        ui.push(Card::new(inputs));

        // ---- Feedback card ----
        let feedback = vec![
            Element::new(
                Id::new("feedback-label"),
                Label::subheading("Feedback"),
            ),
            Element::new(
                Id::new("progress"),
                Progress::new(0.6),
            ),
            Element::new(
                Id::new("spinner"),
                Spinner::new(24.0),
            ),
            Element::new(
                Id::new("alert"),
                Alert::new(
                    AlertKind::Info,
                    "Info",
                    "This is an informational alert.",
                ),
            ),
            Element::new(
                Id::new("badge"),
                Badge::new("New", Color::TW_INDIGO_500),
            ),
            Element::new(
                Id::new("rating"),
                Rating::new(5, 3.5),
            ),
            Element::new(
                Id::new("gauge"),
                Gauge::new(0.75),
            ),
        ];
        ui.push(Card::new(feedback));

        // ---- Icons card ----
        let icons = vec![
            Element::new(
                Id::new("icons-label"),
                Label::subheading("Icons"),
            ),
            Element::new(
                Id::new("icon-star"),
                Icon::new(IconKind::Star, 32.0).with_color(Color::TW_AMBER_500),
            ),
            Element::new(
                Id::new("icon-heart"),
                Icon::new(IconKind::Heart, 32.0).with_color(Color::TW_ROSE_500),
            ),
            Element::new(
                Id::new("icon-check"),
                Icon::new(IconKind::Check, 32.0).with_color(Color::TW_EMERALD_500),
            ),
            Element::new(
                Id::new("icon-search"),
                Icon::new(IconKind::Search, 32.0).with_color(Color::TW_INDIGO_500),
            ),
            Element::new(
                Id::new("icon-settings"),
                Icon::new(IconKind::Settings, 32.0).with_color(Color::TW_SLATE_700),
            ),
        ];
        ui.push(Card::new(icons));

        // ---- Navigation card ----
        let nav = vec![
            Element::new(
                Id::new("nav-label"),
                Label::subheading("Navigation"),
            ),
            Element::new(
                Id::new("pagination"),
                Pagination::new(10, 2),
            ),
            Element::new(
                Id::new("breadcrumb"),
                Breadcrumb::new(vec![
                    Crumb::new("Home"),
                    Crumb::new("Settings"),
                    Crumb::new("Profile"),
                ]),
            ),
            Element::new(
                Id::new("stepper"),
                Stepper::new(vec![
                    Step::new("Account"),
                    Step::new("Details"),
                    Step::new("Confirm"),
                ]),
            ),
        ];
        ui.push(Card::new(nav));

        // Suppress unused warnings for variants re-exported via prelude.
        let _ = (ButtonVariant::Primary, LabelVariant::Body);
    }

    fn update(_s: &mut Self::State, _m: Self::Message) {}
}

fn main() {
    lumen::platform::run::<Gallery>(
        AppBuilder::new()
            .title("Gallery")
            .size(900, 1000)
            .theme(Theme::light()),
    );
}
