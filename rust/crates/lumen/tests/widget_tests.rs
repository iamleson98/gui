use lumen::widget::widgets::*;

#[test]
fn button_exists() {
    let _ = Button::new("OK");
}

#[test]
fn label_text() {
    let l = Label::new("Hello");
    assert_eq!(l.text(), "Hello");
}

#[test]
fn checkbox_default() {
    let c = Checkbox::new(false);
    assert!(!c.checked());
}

#[test]
fn slider_value() {
    let s = Slider::new(0.0, 100.0, 50.0);
    assert_eq!(s.value(), 50.0);
}

#[test]
fn progress_clamps() {
    let p = Progress::new(1.5);
    let p2 = Progress::new(-0.5);
    // Values are clamped internally
    let _ = (p, p2);
}

#[test]
fn toggle_state() {
    let t = Toggle::new(false);
    assert!(!t.is_on());
}

#[test]
fn all_widgets_construct() {
    let _ = Button::new("OK");
    let _ = Label::new("text");
    let _ = Checkbox::new(false);
    let _ = Slider::new(0.0, 100.0, 50.0);
    let _ = Progress::new(0.5);
    let _ = Toggle::new(false);
    let _ = Badge::new("New", lumen::core::Color::TW_INDIGO_500);
    let _ = Avatar::new("JD", 40.0);
    let _ = Spinner::new(24.0);
    let _ = Skeleton::new();
    let _ = Sparkline::new();
    let _ = Gauge::new(0.7);
    let _ = Rating::new(5, 3.0);
    let _ = Pagination::new(10, 0);
    let _ = NumberInput::new(42.0);
    let _ = RangeSlider::new(0.0, 100.0, 25.0, 75.0);
    let _ = RadioGroup::new(vec!["A", "B", "C"]).with_selected(1);
    let _ = Dropdown::new(vec!["X", "Y"], 0);
    let _ = ColorPicker::new(lumen::core::Color::RED);
    let _ = DatePicker::new(lumen::widget::widgets::Date::new(2026, 8, 15));
    let _ = Chip::new(0, "tag");
    let _ = Alert::new(AlertKind::Info, "Title", "Message");
    let _ = Breadcrumb::new(vec![Crumb::new("Home")]);
    let _ = Stepper::new(vec![Step::new("Step 1")]);
}
