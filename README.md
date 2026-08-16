# lumen

Cross-platform GPU-accelerated GUI libraries in **Rust** and **C++**.

## Features

- **36 widgets** per language (Button, Label, TextInput, Checkbox, Slider, Toggle, Progress, Badge, Avatar, Scroll, Card, Canvas, Select, Dropdown, RadioGroup, Tabs, Accordion, Tooltip, Dialog, NumberInput, Table, Tree, DatePicker, ColorPicker, Stepper, RangeSlider, Rating, Pagination, Alert, Chip, Spinner, Skeleton, Sparkline, Gauge, Breadcrumb, Container)
- **Tailwind-style styling** — `Style::new().px_4().py_2().rounded_md().bg_primary().text_white()`
- **Flex + Grid layout engine** — two-pass measure/arrange
- **GPU rendering** — Rust uses wgpu (Vulkan/Metal/DX12), C++ uses OpenGL 3.3
- **Cross-platform** — Linux, macOS, Windows
- **Each widget has its own file** — clean, maintainable structure
- **GitHub Actions CI** — automated testing on every push

## Structure

```
gui/
├── rust/           # Rust version (wgpu + winit)
│   ├── crates/lumen/src/
│   │   ├── core/       # Color, Vec2, Rect, Id
│   │   ├── style/      # Style builder + Theme + Tailwind parser
│   │   ├── layout/     # Flex + Grid layout engine
│   │   ├── render/     # Mesh + Painter + wgpu backend
│   │   ├── event/      # Event system + MessageBus
│   │   ├── input/      # KeyCode, MouseButton, Modifiers
│   │   ├── widget/widgets/  # 36 individual widget files
│   │   └── platform/   # winit ApplicationHandler
│   ├── examples/       # counter, gallery, stress, themes
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
cargo test                     # Run tests (21 tests)
```

### C++
```bash
cd cpp
make                           # Build library + examples
./counter                      # Run counter
```

## Widget List (36)

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
