#pragma once
#include <string>
namespace lumen::widgets {

/// Built-in icon path constants (Heroicons / Lucide style, 24x24 viewBox).
/// Synced with Rust widget/widgets/icons.rs.
enum class IconKind {
    Check, CheckCircle, X, XCircle, Plus, Minus,
    ChevronDown, ChevronUp, ChevronLeft, ChevronRight,
    ArrowRight, ArrowLeft, ArrowUp, ArrowDown,
    Heart, Star, Search, Settings, Home, User, Bell, Mail, Calendar, Clock,
    Trash, Edit, Save, Download, Upload, Eye, EyeOff, Lock, Unlock,
    Menu, Close, Info, Warning, Error, Sun, Moon, Cloud, Folder, File,
    Refresh, Spinner, Play, Pause, Stop, SkipForward, SkipBack,
    Volume, VolumeMute,
};

/// Get the SVG path string for a built-in icon.
inline const char* icon_path(IconKind k) {
    switch (k) {
        case IconKind::Check: return "M 5 13 L 9 17 L 19 7 L 17 5 L 9 13 L 7 11 Z";
        case IconKind::CheckCircle: return "M 12 2 A 10 10 0 1 0 12 22 A 10 10 0 1 0 12 2 Z M 10 14 L 7 11 L 5 13 L 10 18 L 19 9 L 17 7 Z";
        case IconKind::X: return "M 6 6 L 18 18 M 18 6 L 6 18";
        case IconKind::XCircle: return "M 12 2 A 10 10 0 1 0 12 22 A 10 10 0 1 0 12 2 Z M 8 8 L 16 16 M 16 8 L 8 16";
        case IconKind::Plus: return "M 11 5 L 13 5 L 13 11 L 19 11 L 19 13 L 13 13 L 13 19 L 11 19 L 11 13 L 5 13 L 5 11 L 11 11 Z";
        case IconKind::Minus: return "M 5 11 L 19 11 L 19 13 L 5 13 Z";
        case IconKind::ChevronDown: return "M 6 9 L 12 15 L 18 9 L 16 7 L 12 11 L 8 7 Z";
        case IconKind::ChevronUp: return "M 6 15 L 12 9 L 18 15 L 16 17 L 12 13 L 8 17 Z";
        case IconKind::ChevronLeft: return "M 15 6 L 9 12 L 15 18 L 17 16 L 13 12 L 17 8 Z";
        case IconKind::ChevronRight: return "M 9 6 L 15 12 L 9 18 L 7 16 L 11 12 L 7 8 Z";
        case IconKind::ArrowRight: return "M 4 11 L 16 11 L 16 7 L 22 12 L 16 17 L 16 13 L 4 13 Z";
        case IconKind::ArrowLeft: return "M 20 11 L 8 11 L 8 7 L 2 12 L 8 17 L 8 13 L 20 13 Z";
        case IconKind::ArrowUp: return "M 11 20 L 11 8 L 7 8 L 12 2 L 17 8 L 13 8 L 13 20 Z";
        case IconKind::ArrowDown: return "M 11 4 L 11 16 L 7 16 L 12 22 L 17 16 L 13 16 L 13 4 Z";
        case IconKind::Heart: return "M 12 21 L 10 19 C 4 14 2 11 2 8 C 2 5 4 3 7 3 C 9 3 11 4 12 6 C 13 4 15 3 17 3 C 20 3 22 5 22 8 C 22 11 20 14 14 19 Z";
        case IconKind::Star: return "M 12 2 L 15 9 L 22 10 L 17 15 L 18 22 L 12 19 L 6 22 L 7 15 L 2 10 L 9 9 Z";
        case IconKind::Search: return "M 10 2 A 8 8 0 1 0 10 18 A 8 8 0 1 0 10 2 Z M 16 16 L 22 22 L 20 22 L 14 16 Z";
        case IconKind::Settings: return "M 12 8 A 4 4 0 1 0 12 16 A 4 4 0 1 0 12 8 Z M 19 12 C 19 13 21 14 21 16 L 19 19 L 17 18 C 16 19 15 19 14 20 L 13 22 L 11 22 L 10 20 C 9 19 8 19 7 18 L 5 19 L 3 16 C 3 14 5 13 5 12 C 5 11 3 10 3 8 L 5 5 L 7 6 C 8 5 9 5 10 4 L 11 2 L 13 2 L 14 4 C 15 5 16 5 17 6 L 19 5 L 21 8 C 21 10 19 11 19 12 Z";
        case IconKind::Home: return "M 3 12 L 12 3 L 21 12 L 19 12 L 19 21 L 14 21 L 14 14 L 10 14 L 10 21 L 5 21 L 5 12 Z";
        case IconKind::User: return "M 12 2 A 5 5 0 1 0 12 12 A 5 5 0 1 0 12 2 Z M 4 22 C 4 17 7 14 12 14 C 17 14 20 17 20 22 Z";
        case IconKind::Bell: return "M 12 2 C 8 2 6 5 6 9 C 6 13 4 15 4 15 L 20 15 C 20 15 18 13 18 9 C 18 5 16 2 12 2 Z M 10 18 C 10 20 11 21 12 21 C 13 21 14 20 14 18 Z";
        case IconKind::Mail: return "M 2 5 L 22 5 L 22 19 L 2 19 Z M 2 6 L 12 13 L 22 6";
        case IconKind::Calendar: return "M 3 5 L 21 5 L 21 21 L 3 21 Z M 3 9 L 21 9 M 8 3 L 8 7 M 16 3 L 16 7";
        case IconKind::Clock: return "M 12 2 A 10 10 0 1 0 12 22 A 10 10 0 1 0 12 2 Z M 12 6 L 12 12 L 16 14";
        case IconKind::Trash: return "M 4 7 L 20 7 L 18 21 L 6 21 Z M 9 4 L 15 4 L 15 6 L 9 6 Z M 10 10 L 10 18 M 14 10 L 14 18";
        case IconKind::Edit: return "M 3 17 L 3 21 L 7 21 L 19 9 L 15 5 L 3 17 Z M 14 6 L 18 10";
        case IconKind::Save: return "M 5 3 L 19 3 L 21 5 L 21 21 L 3 21 L 3 5 Z M 8 3 L 16 3 L 16 9 L 8 9 Z M 8 14 L 16 14 L 16 21 L 8 21 Z";
        case IconKind::Download: return "M 12 2 L 12 16 M 6 10 L 12 16 L 18 10 M 4 20 L 20 20";
        case IconKind::Upload: return "M 12 22 L 12 8 M 6 14 L 12 8 L 18 14 M 4 4 L 20 4";
        case IconKind::Eye: return "M 2 12 C 6 5 18 5 22 12 C 18 19 6 19 2 12 Z M 12 8 A 4 4 0 1 0 12 16 A 4 4 0 1 0 12 8 Z";
        case IconKind::EyeOff: return "M 3 3 L 21 21 M 10 6 C 14 5 18 7 22 12 C 20 15 17 17 14 18 M 6 6 C 4 8 2 10 2 12 C 6 19 18 19 22 12 M 9 9 C 8 10 8 11 8 12 A 4 4 0 0 0 12 16 C 13 16 14 16 15 15";
        case IconKind::Lock: return "M 5 10 L 19 10 L 19 21 L 5 21 Z M 8 10 L 8 6 A 4 4 0 1 1 16 6 L 16 10";
        case IconKind::Unlock: return "M 5 10 L 19 10 L 19 21 L 5 21 Z M 8 10 L 8 6 A 4 4 0 0 1 16 6 L 16 8";
        case IconKind::Menu: return "M 3 6 L 21 6 M 3 12 L 21 12 M 3 18 L 21 18";
        case IconKind::Close: return "M 6 6 L 18 18 M 18 6 L 6 18";
        case IconKind::Info: return "M 12 2 A 10 10 0 1 0 12 22 A 10 10 0 1 0 12 2 Z M 11 11 L 13 11 L 13 17 L 11 17 Z M 11 7 L 13 7 L 13 9 L 11 9 Z";
        case IconKind::Warning: return "M 12 2 L 22 20 L 2 20 Z M 11 9 L 13 9 L 13 15 L 11 15 Z M 11 16 L 13 16 L 13 18 L 11 18 Z";
        case IconKind::Error: return "M 12 2 A 10 10 0 1 0 12 22 A 10 10 0 1 0 12 2 Z M 11 7 L 13 7 L 13 13 L 11 13 Z M 11 15 L 13 15 L 13 17 L 11 17 Z";
        case IconKind::Sun: return "M 12 7 A 5 5 0 1 0 12 17 A 5 5 0 1 0 12 7 Z M 12 1 L 12 4 M 12 20 L 12 23 M 1 12 L 4 12 M 20 12 L 23 12 M 4 4 L 6 6 M 18 18 L 20 20 M 4 20 L 6 18 M 18 6 L 20 4";
        case IconKind::Moon: return "M 21 13 A 9 9 0 0 1 11 3 A 9 9 0 1 0 21 13 Z";
        case IconKind::Cloud: return "M 7 18 C 4 18 2 16 2 13 C 2 10 4 8 7 8 C 8 5 11 3 14 3 C 18 3 21 6 21 10 C 23 10 24 12 24 14 C 24 16 22 18 20 18 Z";
        case IconKind::Folder: return "M 3 5 L 10 5 L 12 7 L 21 7 L 21 20 L 3 20 Z";
        case IconKind::File: return "M 6 2 L 14 2 L 20 8 L 20 22 L 6 22 Z M 14 2 L 14 8 L 20 8";
        case IconKind::Refresh: return "M 20 12 A 8 8 0 1 1 12 4 L 12 2 L 16 5 L 12 8 L 12 6 A 6 6 0 1 0 18 12 Z";
        case IconKind::Spinner: return "M 12 2 A 10 10 0 1 0 12 22 A 10 10 0 1 0 12 2 Z M 12 4 A 8 8 0 1 1 12 20";
        case IconKind::Play: return "M 5 3 L 20 12 L 5 21 Z";
        case IconKind::Pause: return "M 6 4 L 10 4 L 10 20 L 6 20 Z M 14 4 L 18 4 L 18 20 L 14 20 Z";
        case IconKind::Stop: return "M 5 5 L 19 5 L 19 19 L 5 19 Z";
        case IconKind::SkipForward: return "M 4 5 L 14 12 L 4 19 Z M 16 5 L 20 5 L 20 19 L 16 19 Z";
        case IconKind::SkipBack: return "M 20 5 L 10 12 L 20 19 Z M 4 5 L 8 5 L 8 19 L 4 19 Z";
        case IconKind::Volume: return "M 4 9 L 4 15 L 9 15 L 14 19 L 14 5 L 9 9 Z M 16 8 C 18 9 19 11 19 12 C 19 13 18 15 16 16";
        case IconKind::VolumeMute: return "M 4 9 L 4 15 L 9 15 L 14 19 L 14 5 L 9 9 Z M 16 9 L 20 15 M 20 9 L 16 15";
    }
    return "";
}

} // namespace lumen::widgets
