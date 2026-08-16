# lumen

Cross-platform GPU-accelerated GUI libraries in Rust and C++.

## Structure

- `rust/` — Rust version (wgpu + winit, 40+ widgets, Tailwind styling)
- `cpp/` — C++ version (OpenGL 3.3 + X11, 20+ widgets, same API design)

## Rust

```bash
cd rust
cargo run --example counter
```

## C++

```bash
cd cpp
make
./counter
```

## License

MIT or Apache-2.0.
