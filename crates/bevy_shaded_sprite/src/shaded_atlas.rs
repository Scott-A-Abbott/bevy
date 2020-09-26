use crate::Rect;
use bevy_asset::Handle;
use bevy_core::Bytes;
use bevy_math::Vec2;
use bevy_render::{
    color::Color,
    renderer::{RenderResource, RenderResources},
    texture::Texture,
};
/// An atlas containing multiple textures (like a spritesheet or a tilemap)
#[derive(RenderResources)]
pub struct ShadedAtlas {
    /// The handle to the texture in which the sprites are stored
    pub albedo: Handle<Texture>,
    /// The handle to the texture in which the sprite normal maps are strored.
    pub normal_map: Handle<Texture>,
    // TODO: add support to Uniforms derive to write dimensions and sprites to the same buffer
    pub size: Vec2,
    /// The specific areas of the atlas where each texture can be found
    #[render_resources(buffer)]
    pub frames: Vec<Rect>,
}

// NOTE: cannot do `unsafe impl Byteable` here because Vec3 takes up the space of a Vec4. If/when glam changes this we can swap out
// Bytes for Byteable as a micro-optimization. https://github.com/bitshifter/glam-rs/issues/36
#[derive(Bytes, RenderResources, RenderResource)]
#[render_resources(from_self)]
pub struct ShadedAtlasSprite {
    pub color: Color,
    pub index: u32,
}

impl Default for ShadedAtlasSprite {
    fn default() -> Self {
        Self {
            index: 0,
            color: Color::WHITE,
        }
    }
}

impl ShadedAtlasSprite {
    pub fn new(index: u32) -> ShadedAtlasSprite {
        Self {
            index,
            ..Default::default()
        }
    }
}

impl ShadedAtlas {
    /// Create a new `ShadedAtlas` that has a texture, but does not have
    /// any individual sprites specified
    pub fn new_empty(
        albedo: Handle<Texture>,
        normal_map: Handle<Texture>,
        dimensions: Vec2,
    ) -> Self {
        Self {
            albedo,
            normal_map,
            size: dimensions,
            frames: Vec::new(),
        }
    }

    /// Generate a `ShadedAtlas` by splitting a texture into a grid where each
    /// cell of the grid is one of the textures in the atlas
    pub fn from_grid(
        albedo: Handle<Texture>,
        normal_map: Handle<Texture>,
        size: Vec2,
        columns: usize,
        rows: usize,
    ) -> ShadedAtlas {
        let texture_width = size.x() / columns as f32;
        let texture_height = size.y() / rows as f32;
        let mut frames = Vec::new();
        for y in 0..rows {
            for x in 0..columns {
                frames.push(Rect {
                    min: Vec2::new(x as f32 * texture_width, y as f32 * texture_height),
                    max: Vec2::new(
                        (x + 1) as f32 * texture_width,
                        (y + 1) as f32 * texture_height,
                    ),
                })
            }
        }
        ShadedAtlas {
            albedo,
            normal_map,
            size,
            frames,
        }
    }

    /// Add a sprite to the list of textures in the `ShadedAtlas`
    ///
    /// # Arguments
    ///
    /// * `rect` - The section of the atlas that contains the texture to be added,
    /// from the top-left corner of the texture to the bottom-right corner
    pub fn add_frame(&mut self, rect: Rect) {
        self.frames.push(rect);
    }

    /// How many textures are in the `ShadedAtlas`
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}
