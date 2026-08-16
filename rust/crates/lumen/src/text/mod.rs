use crate::core::{Color, Rect, Vec2};
use cosmic_text::{FontSystem, SwashCache};
use std::collections::HashMap;

pub struct TextEngine { pub font_system: FontSystem, pub swash_cache: SwashCache }
impl TextEngine { pub fn new() -> Self { Self { font_system: FontSystem::new(), swash_cache: SwashCache::new() } } }
impl Default for TextEngine { fn default() -> Self { Self::new() } }
