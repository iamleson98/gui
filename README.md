# lumen

Cross-platform GPU-accelerated GUI libraries in **Rust** and **C++**.

## Features

- **37 widgets** per language (Button, Label, TextInput, Checkbox, Slider, Toggle, Progress, Badge, Avatar, Scroll, Card, Canvas, Icon, Select, Dropdown, RadioGroup, Tabs, Accordion, Tooltip, Dialog, NumberInput, Table, Tree, DatePicker, ColorPicker, Stepper, RangeSlider, Rating, Pagination, Alert, Chip, Spinner, Skeleton, Sparkline, Gauge, Breadcrumb, Container)
- **Real text rendering** — Rust uses cosmic-text for font shaping + swash for rasterization, uploaded to a GPU glyph atlas (R8 texture). System fonts are loaded automatically.
- **Custom font loading** — `text.add_font_bytes(include_bytes!("...") .to_vec())` embeds TTF/OTF/WOFF fonts; `set_sans_serif_family(...)` / `set_monospace_family(...)` / `set_serif_family(...)` configure defaults.
- **SVG icon support** — built-in `Icon` widget renders SVG path data (`M`/`L`/`H`/`V`/`C`/`Q`/`A`/`Z`, absolute and relative) directly into the GPU mesh as filled triangles. Ships with 50+ built-in icons via `IconKind`.
- **Refined visual design** — soft drop shadows, rounded-xl corners, indigo-600 primary color, slate scale for text/borders, 3 button variants (Primary / Secondary / Ghost), 4 label variants (Heading / Subheading / Body / Caption), Card widget with border + shadow.
- **Tailwind-style styling** — `Style::new().px_6().py_3().rounded_lg().bg_primary().text_white().font_medium()`
- **Flex + Grid layout engine** — two-pass measure/arrange with text measurement via cosmic-text
- **GPU rendering** — Rust uses wgpu (Vulkan/Metal/DX12), C++ uses OpenGL 3.3
- **Cross-platform** — Linux, macOS, Windows
- **Each widget has its own file** — clean, maintainable structure
- **GitHub Actions CI** — automated testing on every push

## Text rendering

Text is rendered with **cosmic-text** (shaping) + **swash** (rasterization) into a 1024×1024 R8 glyph atlas that is uploaded to the GPU each frame. The wgpu fragment shader samples the atlas as an alpha mask and multiplies by the requested color, so any color of text is supported without re-rasterizing.

```rust
// Use system fonts (automatic)
ui.push(Label::new("Hello, world!"));

// Or embed your own font:
text.add_font_bytes(include_bytes!("../assets/Inter-Regular.ttf").to_vec());
text.set_sans_serif_family("Inter");
```

## SVG icons

```rust
use lumen::widget::widgets::{Icon, IconKind};

// Built-in icon
ui.push(Icon::new(IconKind::Star, 32.0).with_color(Color::TW_AMBER_500));

// Custom SVG path (24×24 viewBox assumed)
ui.push(Icon::from_path(
    "M 12 2 L 15 9 L 22 10 L 17 15 L 18 22 L 12 19 L 6 22 L 7 15 L 2 10 L 9 9 Z",
    64.0,
).with_color(Color::TW_INDIGO_500));
```

Built-in icons (50+): `Check`, `CheckCircle`, `X`, `XCircle`, `Plus`, `Minus`, `ChevronUp/Down/Left/Right`, `ArrowUp/Down/Left/Right`, `Heart`, `Star`, `Search`, `Settings`, `Home`, `User`, `Bell`, `Mail`, `Calendar`, `Clock`, `Trash`, `Edit`, `Save`, `Download`, `Upload`, `Eye`, `EyeOff`, `Lock`, `Unlock`, `Menu`, `Close`, `Info`, `Warning`, `Error`, `Sun`, `Moon`, `Cloud`, `Folder`, `File`, `Refresh`, `Spinner`, `Play`, `Pause`, `Stop`, `SkipForward`, `SkipBack`, `Volume`, `VolumeMute`.

## Structure

```
gui/
├── rust/           # Rust version (wgpu + winit)
│   ├── crates/lumen/src/
│   │   ├── core/       # Color, Vec2, Rect, Id
│   │   ├── style/      # Style builder + Theme + Tailwind parser
│   │   ├── layout/     # Flex + Grid layout engine
│   │   ├── render/     # Mesh + Painter + wgpu backend + svg path parser
│   │   ├── text/       # cosmic-text + glyph atlas
│   │   ├── event/      # Event system + MessageBus
│   │   ├── input/      # KeyCode, MouseButton, Modifiers
│   │   ├── widget/widgets/  # 37 individual widget files (incl. Icon + icons)
│   │   └── platform/   # winit ApplicationHandler
│   ├── examples/       # counter, gallery, stress, themes, icons
│   └── tests/          # unit tests
├── cpp/            # C++ version (OpenGL + X11)
│   ├── include/lumen/
│   │   ├── core/       # Color, Vec2, Rect, Id
│   │   ├── style/      # Style + Theme
│   │   ├── layout/     # Flex layout
│   │   ├── render/     # Mesh + Painter + GL backend
│   │   ├── event/      # Event system
│   │   ├── input/      # Input types
│   │   ├── widget/widgets/  # 36 individual widget headers
│   │   └── platform/   # App entry point
│   ├── src/            # Implementation files
│   ├── examples/       # counter, gallery, stress, themes
│   └── Makefile        # Cross-platform build
└── .github/workflows/  # CI for Rust + C++
```

## Build

### Rust
```bash
cd rust
cargo run --example counter    # Run counter
cargo run --example icons      # Run icon gallery
cargo test                     # Run tests (87 tests)
```

### C++
```bash
cd cpp
make                           # Build library + examples
./counter                      # Run counter
```

## Widget List (37)

| Widget | Rust | C++ |
|--------|------|-----|
| Button | ✅ | ✅ |
| Label | ✅ | ✅ |
| TextInput | ✅ | ✅ |
| Checkbox | ✅ | ✅ |
| Slider | ✅ | ✅ |
| Toggle | ✅ | ✅ |
| Progress | ✅ | ✅ |
| Badge | ✅ | ✅ |
| Avatar | ✅ | ✅ |
| Scroll | ✅ | ✅ |
| Card | ✅ | ✅ |
| Canvas | ✅ | ✅ |
| Icon | ✅ | ✅ |
| Container | ✅ | ✅ |
| Select | ✅ | ✅ |
| Dropdown | ✅ | ✅ |
| RadioGroup | ✅ | ✅ |
| Tabs | ✅ | ✅ |
| Accordion | ✅ | ✅ |
| Tooltip | ✅ | ✅ |
| Dialog | ✅ | ✅ |
| NumberInput | ✅ | ✅ |
| Table | ✅ | ✅ |
| Tree | ✅ | ✅ |
| DatePicker | ✅ | ✅ |
| ColorPicker | ✅ | ✅ |
| Stepper | ✅ | ✅ |
| RangeSlider | ✅ | ✅ |
| Rating | ✅ | ✅ |
| Pagination | ✅ | ✅ |
| Alert | ✅ | ✅ |
| Chip | ✅ | ✅ |
| Spinner | ✅ | ✅ |
| Skeleton | ✅ | ✅ |
| Sparkline | ✅ | ✅ |
| Gauge | ✅ | ✅ |
| Breadcrumb | ✅ | ✅ |

## License

MIT or Apache-2.0.
