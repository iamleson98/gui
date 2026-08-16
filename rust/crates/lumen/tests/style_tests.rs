use lumen::style::{Style, Tw, Theme, Display, FlexDirection, Align};

#[test]
fn style_builder_padding() {
    let s = Style::new().px_4().py_2().build();
    assert_eq!(s.padding.left, 16.0);
    assert_eq!(s.padding.top, 8.0);
}

#[test]
fn style_builder_flex() {
    let s = Style::new().flex().flex_col().items_center().justify_center().build();
    assert_eq!(s.display, Display::Flex);
    assert_eq!(s.flex_direction, FlexDirection::Column);
    assert_eq!(s.align_items, Align::Center);
}

#[test]
fn theme_kinds() {
    assert_eq!(Theme::light().kind, lumen::style::ThemeKind::Light);
    assert_eq!(Theme::dark().kind, lumen::style::ThemeKind::Dark);
}
