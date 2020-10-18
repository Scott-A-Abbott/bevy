#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 3) in vec4 Vertex_Tangent;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out vec4 v_Color;
layout(location = 2) out vec3 v_Position;
layout(location = 3) out vec3 v_Normal;
layout(location = 4) out vec4 v_Tangent;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

// TODO: merge dimensions into "sprites" buffer when that is supported in the Uniforms derive abstraction
layout(set = 2, binding = 0) uniform ShadedAtlas_size {
    vec2 AtlasSize;
};

struct Rect {
    vec2 begin;
    vec2 end;
};

layout(set = 2, binding = 1) buffer ShadedAtlas_frames {
    Rect[] Frames;
};


layout(set = 3, binding = 0) uniform Transform {
    mat4 SpriteTransform;
};

layout(set = 3, binding = 1) uniform ShadedAtlasSprite {
    vec4 ShadedAtlasSprite_color;
    uint ShadedAtlasSprite_index;
    bool ShadedAtlasSprite_flip;
};

void main() {
    Rect sprite_rect = Frames[ShadedAtlasSprite_index];
    vec2 sprite_dimensions = sprite_rect.end - sprite_rect.begin;
    vec2 atlas_positions[4] = vec2[](
        vec2(sprite_rect.begin.x, sprite_rect.end.y),
        sprite_rect.begin,
        vec2(sprite_rect.end.x, sprite_rect.begin.y), 
        sprite_rect.end
    );
    v_Uv = (atlas_positions[gl_VertexIndex] + vec2(0.01, 0.01)) / AtlasSize;
    v_Color = ShadedAtlasSprite_color;
    v_Tangent = Vertex_Tangent;
    v_Position = ceil(vec3(Vertex_Position.xy * sprite_dimensions, Vertex_Position.z));
    v_Normal = (SpriteTransform * vec4(Vertex_Normal, 1.0)).xyz;
    v_Normal = mat3(SpriteTransform) * Vertex_Normal;
    gl_Position = ViewProj * SpriteTransform * vec4(v_Position, 1.0);
}