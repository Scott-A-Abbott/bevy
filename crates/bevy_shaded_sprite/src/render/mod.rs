use crate::{ShadedAtlas, ShadedAtlasSprite};
use bevy_asset::{Assets, Handle};
use bevy_ecs::Resources;
use bevy_render::{
    pipeline::{
        BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite,
        CompareFunction, CullMode, DepthStencilStateDescriptor, FrontFace, PipelineDescriptor,
        RasterizationStateDescriptor, StencilStateDescriptor, StencilStateFaceDescriptor,
    },
    render_graph::{AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
    shader::{Shader, ShaderStage, ShaderStages},
    texture::TextureFormat,
};
pub const SHADED_SHEET_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(91168858051802816124217444474933884151);

pub fn build_shaded_sheet_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        color_states: vec![ColorStateDescriptor {
            format: TextureFormat::Bgra8UnormSrgb,
            color_blend: BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("shaded_sheet.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("shaded_sheet.frag"),
            ))),
        })
    }
}

pub mod node {
    pub const SHADED_SHEET: &str = "shaded_sheet";
    pub const SHADED_SHEET_SPRITE: &str = "shaded_sheet_sprite";
}

pub(crate) fn add_shaded_atlas_graph(graph: &mut RenderGraph, resources: &Resources) {
    graph.add_system_node(
        node::SHADED_SHEET,
        AssetRenderResourcesNode::<ShadedAtlas>::new(false),
    );

    graph.add_system_node(
        node::SHADED_SHEET_SPRITE,
        RenderResourcesNode::<ShadedAtlasSprite>::new(true),
    );

    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    pipelines.set(
        SHADED_SHEET_PIPELINE_HANDLE,
        build_shaded_sheet_pipeline(&mut shaders),
    );
}
