use crate::{ShadedAtlas, ShadedAtlasSprite, QUAD_HANDLE, SHADED_SHEET_PIPELINE_HANDLE};
use bevy_asset::Handle;
use bevy_ecs::Bundle;
use bevy_render::{
    mesh::Mesh,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
    prelude::Draw,
    render_graph::base::MainPass,
};
use bevy_transform::prelude::{Rotation, Scale, Transform, Translation};

/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `ShadedAtlas`)
#[derive(Bundle)]
pub struct ShadedSheetComponents {
    /// The specific sprite from the texture atlas to be drawn
    pub sprite: ShadedAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub shaded_atlas: Handle<ShadedAtlas>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for ShadedSheetComponents {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                SHADED_SHEET_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 3,
                            binding: 0,
                        },
                        // ShadedAtlasSprite
                        DynamicBinding {
                            bind_group: 3,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            mesh: QUAD_HANDLE,
            main_pass: MainPass,
            sprite: Default::default(),
            shaded_atlas: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Scale(0.05),
        }
    }
}
