use crate::core::Color;
#[derive(Clone, Debug)]
pub struct Palette {
    pub bg: Color, pub surface: Color, pub primary: Color, pub accent: Color,
    pub text: Color, pub text_muted: Color, pub border: Color,
    pub danger: Color, pub success: Color, pub warning: Color, pub info: Color,
}
#[derive(Clone, Debug)]
pub struct Theme { pub kind: ThemeKind, pub palette: Palette, pub density: f32 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeKind { Light, Dark, HighContrast }
impl Default for Theme { fn default() -> Self { Self::light() } }
impl Theme {
    pub fn light() -> Self { Self { kind: ThemeKind::Light, palette: Palette {
        bg: Color::TW_SLATE_100, surface: Color::WHITE, primary: Color::TW_INDIGO_500, accent: Color::TW_EMERALD_500,
        text: Color::TW_SLATE_900, text_muted: Color::rgb(100,116,139), border: Color::rgb(226,232,240),
        danger: Color::TW_ROSE_500, success: Color::TW_EMERALD_500, warning: Color::TW_AMBER_500, info: Color::TW_INDIGO_500,
    }, density: 1.0 }}
    pub fn dark() -> Self { Self { kind: ThemeKind::Dark, palette: Palette {
        bg: Color::rgb(9,11,16), surface: Color::rgb(20,24,32), primary: Color::TW_INDIGO_500, accent: Color::TW_EMERALD_500,
        text: Color::rgb(241,245,249), text_muted: Color::rgb(148,163,184), border: Color::rgb(45,53,67),
        danger: Color::TW_ROSE_500, success: Color::TW_EMERALD_500, warning: Color::TW_AMBER_500, info: Color::TW_INDIGO_500,
    }, density: 1.0 }}
}
