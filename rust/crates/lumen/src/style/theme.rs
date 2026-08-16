use crate::core::Color;

/// The color palette for a theme. All colors are in sRGB space.
#[derive(Clone, Debug)]
pub struct Palette {
    /// App background (the canvas behind all content).
    pub bg: Color,
    /// Elevated surface (cards, panels).
    pub surface: Color,
    /// Even more elevated surface (modals, popovers).
    pub surface_elevated: Color,
    /// Muted background for subtle separators / hover states.
    pub muted: Color,
    /// Primary brand color — buttons, links, focus rings.
    pub primary: Color,
    /// Hover state for primary-colored elements.
    pub primary_hover: Color,
    /// Active/pressed state for primary-colored elements.
    pub primary_active: Color,
    /// Secondary accent color.
    pub accent: Color,
    /// Primary text color.
    pub text: Color,
    /// Muted/secondary text color.
    pub text_muted: Color,
    /// Border / divider color.
    pub border: Color,
    /// Shadow color (semi-transparent black).
    pub shadow: Color,
    pub danger: Color,
    pub success: Color,
    pub warning: Color,
    pub info: Color,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub kind: ThemeKind,
    pub palette: Palette,
    pub density: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeKind {
    Light,
    Dark,
    HighContrast,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    /// A modern, soft light theme. Background is a warm off-white, surfaces
    /// are pure white, primary is indigo-600 for good contrast with white
    /// text.
    pub fn light() -> Self {
        Self {
            kind: ThemeKind::Light,
            palette: Palette {
                bg: Color::TW_SLATE_100,
                surface: Color::WHITE,
                surface_elevated: Color::WHITE,
                muted: Color::TW_SLATE_50,
                primary: Color::TW_INDIGO_600,
                primary_hover: Color::TW_INDIGO_500,
                primary_active: Color::TW_INDIGO_700,
                accent: Color::TW_EMERALD_500,
                text: Color::TW_SLATE_900,
                text_muted: Color::TW_SLATE_500,
                border: Color::TW_SLATE_200,
                shadow: Color::rgba(15, 23, 42, 40),
                danger: Color::TW_ROSE_500,
                success: Color::TW_EMERALD_500,
                warning: Color::TW_AMBER_500,
                info: Color::TW_INDIGO_500,
            },
            density: 1.0,
        }
    }

    /// A deep, modern dark theme. Background is near-black, surfaces are
    /// dark slate, primary stays indigo for brand consistency.
    pub fn dark() -> Self {
        Self {
            kind: ThemeKind::Dark,
            palette: Palette {
                bg: Color::TW_SLATE_900,
                surface: Color::TW_SLATE_800,
                surface_elevated: Color::TW_SLATE_700,
                muted: Color::TW_SLATE_800,
                primary: Color::TW_INDIGO_500,
                primary_hover: Color::TW_INDIGO_400,
                primary_active: Color::TW_INDIGO_600,
                accent: Color::TW_EMERALD_400,
                text: Color::TW_SLATE_50,
                text_muted: Color::TW_SLATE_400,
                border: Color::TW_SLATE_700,
                shadow: Color::rgba(0, 0, 0, 120),
                danger: Color::TW_ROSE_500,
                success: Color::TW_EMERALD_500,
                warning: Color::TW_AMBER_500,
                info: Color::TW_INDIGO_500,
            },
            density: 1.0,
        }
    }
}
