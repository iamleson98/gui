#pragma once
#include <cstdint>
namespace lumen {
enum class Modifiers : uint8_t { None=0, Shift=1, Control=2, Alt=4, Super=8 };
inline Modifiers operator|(Modifiers a, Modifiers b) { return (Modifiers)((uint8_t)a|(uint8_t)b); }
inline bool has_shift(Modifiers m) { return (uint8_t)m&1; }
inline bool has_ctrl(Modifiers m) { return (uint8_t)m&2; }
enum class MouseButton { Left, Right, Middle, Other };
enum class KeyCode { Escape, Enter, Tab, Backspace, Delete, Insert, Home, End, PageUp, PageDown,
    Left, Right, Up, Down, ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Space, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12 };
} // namespace lumen
