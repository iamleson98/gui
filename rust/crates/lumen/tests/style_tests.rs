use lumen::core::Color;
use lumen::style::{Align, Display, FlexDirection, Style, Theme, Tw};

#[test]
fn style_builder_padding() {
    let s = Style::new().px_4().py_2().build();
    assert_eq!(s.padding.left, 16.0);
    assert_eq!(s.padding.top, 8.0);
}

#[test]
fn style_builder_flex() {
    let s = Style::new()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .build();
    assert_eq!(s.display, Display::Flex);
    assert_eq!(s.flex_direction, FlexDirection::Column);
    assert_eq!(s.align_items, Align::Center);
}

#[test]
fn theme_kinds() {
    assert_eq!(Theme::light().kind, lumen::style::ThemeKind::Light);
    assert_eq!(Theme::dark().kind, lumen::style::ThemeKind::Dark);
}

#[test]
fn style_new_helpers() {
    let s = Style::new().px_6().py_3().rounded_xl().build();
    assert_eq!(s.padding.left, 24.0);
    assert_eq!(s.padding.top, 12.0);
    assert_eq!(s.border_radius.top_left, 14.0);
}

#[test]
fn style_text_base() {
    let s = Style::new().text_base().build();
    assert_eq!(s.font_size, 14.0);
}

#[test]
fn style_text_3xl() {
    let s = Style::new().text_3xl().build();
    assert_eq!(s.font_size, 36.0);
}

#[test]
fn style_min_width_height() {
    let s = Style::new().min_w(100.0).min_h(40.0).build();
    assert_eq!(s.min_width, Some(100.0));
    assert_eq!(s.min_height, Some(40.0));
}

#[test]
fn style_alignment_helpers() {
    let s = Style::new().items_stretch().justify_start().build();
    assert_eq!(s.align_items, Align::Stretch);
    assert_eq!(s.justify_content, Align::Start);
}

#[test]
fn theme_light_has_primary_hover() {
    let t = Theme::light();
    // The refined palette must expose primary_hover / primary_active.
    assert_ne!(t.palette.primary, t.palette.primary_hover);
    assert_ne!(t.palette.primary, t.palette.primary_active);
}

#[test]
fn theme_dark_has_surface() {
    let t = Theme::dark();
    assert_ne!(t.palette.bg, t.palette.surface);
    assert_ne!(t.palette.surface, Color::TRANSPARENT);
}
