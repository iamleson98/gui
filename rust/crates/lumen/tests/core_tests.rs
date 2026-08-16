use lumen::core::{Color, Id, Rect, Vec2};

#[test]
fn color_rgb_preserves_channels() {
    let c = Color::rgb(10, 20, 30);
    assert_eq!([c.r, c.g, c.b, c.a], [10, 20, 30, 255]);
}

#[test]
fn color_hex_parsing() {
    assert_eq!(Color::from_hex("#ff8800"), Some(Color::rgb(255, 136, 0)));
    assert_eq!(Color::from_hex("#FF8800"), Some(Color::rgb(255, 136, 0)));
    assert_eq!(
        Color::from_hex("#ff880080"),
        Some(Color::rgba(255, 136, 0, 128))
    );
    assert_eq!(Color::from_hex("ff8800"), None);
    assert_eq!(Color::from_hex("#gg8800"), None);
}

#[test]
fn color_lerp() {
    assert_eq!(Color::BLACK.lerp(Color::WHITE, 0.0), Color::BLACK);
    assert_eq!(Color::BLACK.lerp(Color::WHITE, 1.0), Color::WHITE);
    assert_eq!(
        Color::BLACK.lerp(Color::WHITE, 0.5),
        Color::rgb(128, 128, 128)
    );
}

#[test]
fn color_with_alpha() {
    assert_eq!(Color::WHITE.with_alpha(50).a, 50);
}

#[test]
fn vec2_arithmetic() {
    assert_eq!(
        Vec2::new(1.0, 2.0) + Vec2::new(3.0, 4.0),
        Vec2::new(4.0, 6.0)
    );
    assert_eq!(
        Vec2::new(5.0, 6.0) - Vec2::new(2.0, 1.0),
        Vec2::new(3.0, 5.0)
    );
    assert_eq!(Vec2::new(2.0, 3.0) * 2.0, Vec2::new(4.0, 6.0));
}

#[test]
fn vec2_dot_length() {
    assert_eq!(Vec2::new(3.0, 4.0).dot(Vec2::new(3.0, 4.0)), 25.0);
    assert!((Vec2::new(3.0, 4.0).length() - 5.0).abs() < 1e-6);
}

#[test]
fn rect_contains() {
    let r = Rect::from_xywh(10.0, 20.0, 30.0, 40.0);
    assert!(r.contains(Vec2::new(25.0, 30.0)));
    assert!(!r.contains(Vec2::new(40.0, 60.0))); // max corner is half-open
}

#[test]
fn rect_intersect() {
    let a = Rect::from_xywh(0.0, 0.0, 10.0, 10.0);
    let b = Rect::from_xywh(5.0, 5.0, 10.0, 10.0);
    assert_eq!(a.intersect(b), Rect::from_xywh(5.0, 5.0, 5.0, 5.0));
}

#[test]
fn id_deterministic() {
    assert_eq!(Id::new("hello"), Id::new("hello"));
    assert_ne!(Id::new("hello"), Id::new("world"));
}

#[test]
fn id_derive() {
    let parent = Id::new("parent");
    assert_eq!(parent.derive("child"), parent.derive("child"));
    assert_ne!(parent.derive("a"), parent.derive("b"));
}

#[test]
fn id_unique() {
    let a = Id::unique();
    let b = Id::unique();
    assert_ne!(a, b);
}
