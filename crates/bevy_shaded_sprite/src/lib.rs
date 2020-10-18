pub mod entity;

mod rect;
mod render;
mod shaded_atlas;
mod shaded_quad;

pub use rect::*;
pub use render::*;
pub use shaded_atlas::*;
pub use shaded_quad::*;

pub mod prelude {
    pub use crate::{entity::ShadedSheetComponents, ShadedAtlas, ShadedAtlasSprite};
}

use bevy_app::prelude::*;
use bevy_asset::{AddAsset, Assets, Handle};
use bevy_render::{mesh::Mesh, render_graph::RenderGraph};

#[derive(Default)]
pub struct ShadedSpritePlugin;

pub const QUAD_HANDLE: Handle<Mesh> = Handle::from_u128(142404619811301375266013514540294236421);

impl Plugin for ShadedSpritePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ShadedAtlas>();

        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        add_shaded_atlas_graph(&mut render_graph, &resources);

        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        meshes.set(
            QUAD_HANDLE,
            Mesh::from(ShadedQuad::new(1.0, 1.0)),
        );
    }
}
