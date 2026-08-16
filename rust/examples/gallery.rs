use lumen::prelude::*;
use lumen::style::{Style, Tw};
struct Gallery;
impl App for Gallery {
    type State = Gallery;
    type Message = ();
    fn init() -> Self::State { Gallery }
    fn view(_s: &Self::State, ui: &mut Ui) {
        // Title
        ui.push(
            Label::new("lumen Widget Gallery")
                .with_style(Style::new().text_xl().font_bold().px_4().py_2().build())
        );
        
        // Buttons
        ui.push(Button::new("Click Me"));
        
        // Inputs
        ui.push(TextInput::new("Type here..."));
        ui.push(Checkbox::new(false));
        ui.push(Toggle::new(true));
        
        // Sliders
        ui.push(Slider::new(0.0, 100.0, 50.0));
        ui.push(RangeSlider::new(0.0, 100.0, 25.0, 75.0));
        
        // Progress
        ui.push(Progress::new(0.6));
        
        // Rating
        ui.push(Rating::new(5, 3.5));
        
        // Number input
        ui.push(NumberInput::new(42.0).with_range(0.0, 100.0).with_step(1.0));
        
        // Badge
        ui.push(Badge::new("New", Color::TW_INDIGO_500));
        
        // Avatar
        ui.push(Avatar::new("JD", 40.0));
        
        // Alert
        ui.push(Alert::new(AlertKind::Info, "Info", "This is an informational alert."));
        
        // Spinner
        ui.push(Spinner::new(24.0));
        
        // Chips
        ui.push(Chip::new(0, "Rust").removable());
        ui.push(Chip::new(1, "GUI").selected());
        
        // Pagination
        ui.push(Pagination::new(10, 2));
        
        // Breadcrumb
        ui.push(Breadcrumb::new(vec![Crumb::new("Home"), Crumb::new("Settings"), Crumb::new("Profile")]));
        
        // Gauge
        ui.push(Gauge::new(0.75));
        
        // Stepper
        ui.push(Stepper::new(vec![Step::new("Account"), Step::new("Details"), Step::new("Confirm")]));
    }
    fn update(_s: &mut Self::State, _m: Self::Message) {}
}
fn main() { lumen::platform::run::<Gallery>(AppBuilder::new().title("Gallery").size(800, 600)); }
