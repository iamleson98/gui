use bitflags::bitflags;
bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Modifiers: u8 { const SHIFT = 1; const CONTROL = 2; const ALT = 4; const SUPER = 8; }
}
impl Modifiers {
    pub fn shift(self) -> bool { self.contains(Self::SHIFT) }
    pub fn ctrl(self) -> bool { self.contains(Self::CONTROL) }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButton { Left, Right, Middle, Other(u8) }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Escape, Enter, Tab, Backspace, Delete, Insert, Home, End, PageUp, PageDown,
    Left, Right, Up, Down, ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Space, Char(char), F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}
