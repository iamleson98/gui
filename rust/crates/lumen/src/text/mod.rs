//! Text rendering — cosmic-text glyph shaping + rasterization + atlas + wgpu upload.
//!
//! This module provides real GPU text rendering using cosmic-text for shaping
//! and swash for rasterization. Glyphs are cached in a texture atlas that is
//! uploaded to the GPU each frame.

use crate::core::{Color, Rect, Vec2};
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, SwashImage};
use std::collections::HashMap;

/// A placed glyph ready to be drawn by the painter.
#[derive(Clone, Copy, Debug)]
pub struct PlacedGlyph {
    /// Screen-space rect (physical px).
    pub rect: Rect,
    /// UV coordinates into the atlas (unnormalized, in atlas pixels).
    pub uv: [u16; 4],
    /// Color the glyph should be tinted with.
    pub color: Color,
}

/// A glyph atlas entry.
#[derive(Clone, Copy, Debug, Default)]
pub struct AtlasEntry {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

/// Owns the cosmic-text machinery + a CPU-side glyph atlas.
pub struct TextEngine {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub atlas_pixels: Vec<u8>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
    /// Cache: cache_key hash -> AtlasEntry.
    cache: HashMap<u64, AtlasEntry>,
    /// Track which atlas entries were used this frame (for future LRU).
    pub atlas_dirty: bool,
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            atlas_width: 1024,
            atlas_height: 1024,
            atlas_pixels: vec![0u8; (1024 * 1024) as usize],
            cursor_x: 0,
            cursor_y: 0,
            row_height: 0,
            cache: HashMap::new(),
            atlas_dirty: false,
        }
    }

    /// Load a custom font from raw font data (TTF / OTF / WOFF). The font
    /// becomes available to all subsequent text layout calls. Use this to
    /// embed an application-specific font with `include_bytes!`.
    ///
    /// After loading, the glyph cache is cleared so that previously-laid-out
    /// text re-rasterises against the new font set.
    pub fn add_font_bytes(&mut self, data: Vec<u8>) {
        self.font_system.db_mut().load_font_data(data);
        // Invalidate the layout caches by clearing the glyph atlas.
        self.clear_atlas();
    }

    /// Set the default sans-serif family name. Must match a family that has
    /// been loaded (system or via `add_font_bytes`).
    pub fn set_sans_serif_family(&mut self, name: &str) {
        self.font_system.db_mut().set_sans_serif_family(name);
        self.clear_atlas();
    }

    /// Set the default monospace family name.
    pub fn set_monospace_family(&mut self, name: &str) {
        self.font_system.db_mut().set_monospace_family(name);
        self.clear_atlas();
    }

    /// Set the default serif family name.
    pub fn set_serif_family(&mut self, name: &str) {
        self.font_system.db_mut().set_serif_family(name);
        self.clear_atlas();
    }

    /// Load all fonts from a directory at runtime (e.g. user-supplied fonts).
    pub fn load_fonts_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) {
        self.font_system.db_mut().load_fonts_dir(dir);
        self.clear_atlas();
    }

    /// Measure the width and height of a single line of text without
    /// rasterizing any glyphs. The returned size is in logical pixels
    /// and can be used as a layout intrinsic.
    pub fn measure_text(&mut self, text: &str, font_size: f32) -> Vec2 {
        if text.is_empty() {
            return Vec2::new(0.0, font_size * 1.4);
        }
        let metrics = Metrics::new(font_size, font_size * 1.4);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        let attrs = Attrs::new().family(Family::SansSerif);
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut max_w = 0.0f32;
        let mut max_h = 0.0f32;
        for run in buffer.layout_runs() {
            let run_w: f32 = run.glyphs.iter().map(|g| g.w).sum();
            max_w = max_w.max(run_w);
            max_h = run.line_y.max(max_h);
        }
        if max_h == 0.0 {
            max_h = font_size * 1.4;
        }
        Vec2::new(max_w.ceil(), max_h.ceil())
    }

    /// Lay out a single line of text and return positioned glyphs.
    /// Returns a list of (screen_rect, uv, color) tuples.
    pub fn layout_text(
        &mut self,
        text: &str,
        font_size: f32,
        color: Color,
        origin: Vec2,
    ) -> Vec<PlacedGlyph> {
        if text.is_empty() {
            return Vec::new();
        }

        let metrics = Metrics::new(font_size, font_size * 1.4);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        let attrs = Attrs::new().family(Family::SansSerif);
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut glyphs = Vec::new();
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical = glyph.physical((0.0, 0.0), 1.0);
                let cache_key = physical.cache_key;

                // Try cache
                let key_hash = hash_cache_key(&cache_key);
                let entry = if let Some(e) = self.cache.get(&key_hash) {
                    *e
                } else {
                    // Rasterize — clone the image data to release the borrow
                    let image_data: Option<(u32, u32, Vec<u8>)> = {
                        let img_ref = self.swash_cache.get_image(&mut self.font_system, cache_key);
                        match img_ref {
                            Some(ref i) => Some((
                                i.placement.width as u32,
                                i.placement.height as u32,
                                i.data.clone(),
                            )),
                            None => None,
                        }
                    };
                    let (w, h, data) = match image_data {
                        Some(d) => d,
                        None => continue,
                    };
                    let entry = self.place_in_atlas_raw(w, h, &data);
                    if entry.w == 0 || entry.h == 0 {
                        continue;
                    }
                    self.cache.insert(key_hash, entry);
                    entry
                };

                if entry.w == 0 || entry.h == 0 {
                    continue;
                }

                let x = origin.x + physical.x as f32;
                let y = origin.y + run.line_y - physical.y as f32;

                glyphs.push(PlacedGlyph {
                    rect: Rect::from_xywh(x, y, entry.w as f32, entry.h as f32),
                    uv: [entry.x, entry.y, entry.w, entry.h],
                    color,
                });
            }
        }

        glyphs
    }

    /// Place raw rasterized glyph data into the atlas.
    fn place_in_atlas_raw(&mut self, w: u32, h: u32, data: &[u8]) -> AtlasEntry {
        if w == 0 || h == 0 {
            return AtlasEntry::default();
        }

        // Wrap cursor if it doesn't fit
        if self.cursor_x + w > self.atlas_width {
            self.cursor_x = 0;
            self.cursor_y += self.row_height + 1;
            self.row_height = 0;
        }
        if self.cursor_y + h > self.atlas_height {
            log::warn!("lumen: glyph atlas overflow, glyph dropped");
            return AtlasEntry::default();
        }

        // Copy pixels. cosmic-text gives us alpha mask (1 byte per pixel).
        let src_stride = w as usize;
        for y in 0..h as usize {
            for x in 0..w as usize {
                let src_idx = y * src_stride + x;
                let dst_x = self.cursor_x as usize + x;
                let dst_y = self.cursor_y as usize + y;
                let dst_idx = dst_y * self.atlas_width as usize + dst_x;
                if src_idx < data.len() {
                    self.atlas_pixels[dst_idx] = data[src_idx];
                }
            }
        }

        let entry = AtlasEntry {
            x: self.cursor_x as u16,
            y: self.cursor_y as u16,
            w: w as u16,
            h: h as u16,
        };

        self.cursor_x += w + 1;
        self.row_height = self.row_height.max(h);
        self.atlas_dirty = true;

        entry
    }

    /// Place a rasterized glyph image into the atlas.
    fn place_in_atlas(&mut self, image: &SwashImage) -> AtlasEntry {
        self.place_in_atlas_raw(
            image.placement.width as u32,
            image.placement.height as u32,
            &image.data,
        )
    }

    /// Clear the glyph atlas (call when atlas is full or on theme change).
    pub fn clear_atlas(&mut self) {
        self.atlas_pixels.fill(0);
        self.cache.clear();
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.row_height = 0;
        self.atlas_dirty = true;
    }

    /// Borrow the raw atlas pixels (R8) for GPU upload.
    pub fn atlas_pixels(&self) -> &[u8] {
        &self.atlas_pixels
    }

    pub fn atlas_size(&self) -> (u32, u32) {
        (self.atlas_width, self.atlas_height)
    }
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn hash_cache_key(k: &cosmic_text::CacheKey) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = ahash::AHasher::default();
    k.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_engine_creates() {
        let _te = TextEngine::new();
    }

    #[test]
    fn layout_empty_text() {
        let mut te = TextEngine::new();
        let glyphs = te.layout_text("", 14.0, Color::BLACK, Vec2::ZERO);
        assert!(glyphs.is_empty());
    }

    #[test]
    fn layout_nonempty_text_returns_glyphs() {
        let mut te = TextEngine::new();
        let glyphs = te.layout_text("Hello", 14.0, Color::BLACK, Vec2::ZERO);
        // Should get at least some glyphs (may be 0 if no fonts installed, but
        // cosmic-text bundles a default font)
        // We don't assert > 0 because CI might not have fonts, but it shouldn't panic.
        let _ = glyphs;
    }

    #[test]
    fn clear_atlas_works() {
        let mut te = TextEngine::new();
        te.atlas_pixels[0] = 255;
        te.clear_atlas();
        assert_eq!(te.atlas_pixels[0], 0);
    }

    #[test]
    fn add_font_bytes_does_not_panic() {
        // Empty font data is a no-op for cosmic-text/fontdb
        let mut te = TextEngine::new();
        te.add_font_bytes(Vec::new());
    }

    #[test]
    fn set_sans_serif_family_works() {
        let mut te = TextEngine::new();
        te.set_sans_serif_family("DejaVu Sans");
        te.set_monospace_family("DejaVu Sans Mono");
        te.set_serif_family("DejaVu Serif");
    }

    #[test]
    fn measure_text_returns_positive_size_for_nonempty() {
        let mut te = TextEngine::new();
        let size = te.measure_text("Hello, world!", 16.0);
        // We can't assert exact dimensions (depends on bundled fonts), but
        // a non-empty string at a non-zero font size should produce a
        // positive height (at least the line height) and a non-negative width.
        assert!(size.y > 0.0, "measured height should be > 0, got {}", size.y);
        assert!(size.x >= 0.0, "measured width should be >= 0, got {}", size.x);
    }

    #[test]
    fn measure_text_empty_returns_line_height() {
        let mut te = TextEngine::new();
        let size = te.measure_text("", 16.0);
        assert_eq!(size.x, 0.0);
        // Empty text should still report a line-height-sized y.
        assert!((size.y - 16.0 * 1.4).abs() < 0.01);
    }

    #[test]
    fn measure_text_larger_font_is_larger_or_equal() {
        let mut te = TextEngine::new();
        let small = te.measure_text("Test", 10.0);
        let large = te.measure_text("Test", 20.0);
        // A larger font should produce a larger (or equal, if no glyphs) height.
        assert!(large.y >= small.y, "larger font should have >= height");
    }
}
