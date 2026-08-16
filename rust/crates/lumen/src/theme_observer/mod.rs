//! System theme observer — detects OS dark/light preference.
//!
//! On platforms where detection isn't possible, defaults to Light.

use crate::style::Theme;

/// System color scheme preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemTheme {
    Light,
    Dark,
    NoPreference,
}

/// Detect the system's color scheme preference.
pub fn detect_system_theme() -> SystemTheme {
    // On Linux, check the GTK/DConf setting
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains("dark") {
                return SystemTheme::Dark;
            }
            if s.contains("light") {
                return SystemTheme::Light;
            }
        }
        // Fallback: check GTK_THEME env
        if let Ok(theme) = std::env::var("GTK_THEME") {
            if theme.to_lowercase().contains("dark") {
                return SystemTheme::Dark;
            }
        }
    }

    // On macOS, check the defaults
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.to_lowercase().contains("dark") {
                return SystemTheme::Dark;
            }
        }
    }

    SystemTheme::NoPreference
}

/// Get the appropriate theme based on system preference.
/// Falls back to light if no preference.
pub fn system_theme() -> Theme {
    match detect_system_theme() {
        SystemTheme::Dark => Theme::dark(),
        _ => Theme::light(),
    }
}

/// A theme manager that can observe system changes and switch themes.
pub struct ThemeManager {
    current: Theme,
    auto_detect: bool,
    on_change: Vec<Box<dyn Fn(&Theme)>>,
}

impl ThemeManager {
    pub fn new(initial: Theme) -> Self {
        Self {
            current: initial,
            auto_detect: false,
            on_change: Vec::new(),
        }
    }

    /// Create a theme manager that auto-detects system theme.
    pub fn auto_detect() -> Self {
        let theme = system_theme();
        Self {
            current: theme,
            auto_detect: true,
            on_change: Vec::new(),
        }
    }

    /// Get the current theme.
    pub fn current(&self) -> &Theme {
        &self.current
    }

    /// Set a specific theme (disables auto-detect).
    pub fn set_theme(&mut self, theme: Theme) {
        self.auto_detect = false;
        self.current = theme;
        self.notify();
    }

    /// Enable auto-detection.
    pub fn enable_auto_detect(&mut self) {
        self.auto_detect = true;
        self.refresh();
    }

    /// Refresh from system (call periodically).
    pub fn refresh(&mut self) {
        if !self.auto_detect {
            return;
        }
        let new = system_theme();
        if new.kind != self.current.kind {
            self.current = new;
            self.notify();
        }
    }

    /// Subscribe to theme changes.
    pub fn on_change<F: Fn(&Theme) + 'static>(&mut self, f: F) {
        self.on_change.push(Box::new(f));
    }

    fn notify(&self) {
        for f in &self.on_change {
            f(&self.current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_valid() {
        let t = detect_system_theme();
        // Should always return something (may be NoPreference)
        let _ = t;
    }

    #[test]
    fn theme_manager_set() {
        let mut tm = ThemeManager::new(Theme::light());
        assert_eq!(tm.current().kind, crate::style::ThemeKind::Light);
        tm.set_theme(Theme::dark());
        assert_eq!(tm.current().kind, crate::style::ThemeKind::Dark);
    }

    #[test]
    fn theme_manager_notifies() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut tm = ThemeManager::new(Theme::light());
        let called = Rc::new(RefCell::new(false));
        let c = called.clone();
        tm.on_change(move |_| { *c.borrow_mut() = true; });
        tm.set_theme(Theme::dark());
        assert!(*called.borrow());
    }

    #[test]
    fn theme_manager_auto_detect() {
        let tm = ThemeManager::auto_detect();
        // Should have a valid theme
        let _ = tm.current().kind;
    }
}
