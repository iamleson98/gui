//! Icon gallery — demonstrates every built-in SVG icon plus a custom path.
//!
//! Run with: `cargo run --example icons`

use lumen::prelude::*;
use lumen::style::{Style, Theme, Tw};
use lumen::widget::widgets::{Icon, IconKind, Label};

struct Icons;

impl App for Icons {
    type State = Icons;
    type Message = ();
    fn init() -> Self::State {
        Icons
    }

    fn view(_s: &Self::State, ui: &mut Ui) {
        ui.push(
            Label::new("lumen Icon Gallery")
                .with_style(Style::new().text_xl().font_bold().px_4().py_2().build()),
        );

        // A selection of built-in icons at 32px, tinted with the primary color.
        let kinds = [
            IconKind::Check,
            IconKind::CheckCircle,
            IconKind::X,
            IconKind::XCircle,
            IconKind::Plus,
            IconKind::Minus,
            IconKind::Search,
            IconKind::Settings,
            IconKind::Home,
            IconKind::User,
            IconKind::Bell,
            IconKind::Mail,
            IconKind::Heart,
            IconKind::Star,
            IconKind::Sun,
            IconKind::Moon,
            IconKind::Lock,
            IconKind::Unlock,
            IconKind::Eye,
            IconKind::EyeOff,
            IconKind::Edit,
            IconKind::Trash,
            IconKind::Save,
            IconKind::Download,
            IconKind::Upload,
            IconKind::Play,
            IconKind::Pause,
            IconKind::Stop,
            IconKind::ArrowRight,
            IconKind::ArrowLeft,
            IconKind::ArrowUp,
            IconKind::ArrowDown,
            IconKind::ChevronRight,
            IconKind::ChevronLeft,
            IconKind::ChevronUp,
            IconKind::ChevronDown,
            IconKind::Info,
            IconKind::Warning,
            IconKind::Error,
            IconKind::Refresh,
            IconKind::Menu,
            IconKind::Close,
            IconKind::Calendar,
            IconKind::Clock,
            IconKind::Folder,
            IconKind::File,
            IconKind::Cloud,
            IconKind::Spinner,
        ];

        for k in kinds.iter() {
            ui.push(Icon::new(*k, 32.0).with_color(Color::TW_INDIGO_500));
        }

        // A custom SVG path (a 5-pointed star variant)
        ui.push(
            Label::new("Custom SVG path:").with_style(Style::new().text_sm().px_4().py_2().build()),
        );
        ui.push(
            Icon::from_path(
                "M 12 1 L 14 9 L 22 9 L 16 14 L 18 22 L 12 17 L 6 22 L 8 14 L 2 9 L 10 9 Z",
                64.0,
            )
            .with_color(Color::TW_AMBER_500),
        );
    }

    fn update(_s: &mut Self::State, _m: Self::Message) {}
}

fn main() {
    lumen::platform::run::<Icons>(
        AppBuilder::new()
            .title("Icons")
            .size(900, 700)
            .theme(Theme::light()),
    );
}
