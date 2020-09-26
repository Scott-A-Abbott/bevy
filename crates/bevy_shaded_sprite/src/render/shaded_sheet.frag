#version 450

const int MAX_LIGHTS = 10;
struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec4 v_Color;
layout(location = 2) in vec3 v_Position;
layout(location = 3) in vec3 v_Normal;
layout(location = 4) in vec4 v_Tangent;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform Lights {
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(set = 2, binding = 2) uniform texture2D ShadedAtlas_albedo;
layout(set = 2, binding = 3) uniform sampler ShadedAtlas_albedo_sampler;

layout(set = 2, binding = 4) uniform texture2D ShadedAtlas_normal_map;
layout(set = 2, binding = 5) uniform sampler ShadedAtlas_normal_map_sampler;

void main() {
    vec3 ambient = vec3(0.05, 0.05, 0.05);
    vec3 tangent = v_Tangent.xyz;
    vec4 albedo = v_Color * texture(
        sampler2D(ShadedAtlas_albedo, ShadedAtlas_albedo_sampler),
        v_Uv);
    vec3 normal_map = texture(
        sampler2D(ShadedAtlas_normal_map, ShadedAtlas_normal_map_sampler),
        v_Uv).rgb;
    normal_map = normal_map * 2 - 1;
    
    vec3 vertex_normal = normalize(v_Normal);
    vec3 vertex_tangent = normalize(tangent - vertex_normal * dot(vertex_normal, tangent));
    vec3 vertex_bitangent = normalize(cross(vertex_normal, vertex_tangent)) * v_Tangent.w;
    mat3 vertex_basis = mat3(
        vertex_tangent.x, vertex_bitangent.x, vertex_normal.x,
        vertex_tangent.y, vertex_bitangent.y, vertex_normal.y,
        vertex_tangent.z, vertex_bitangent.z, vertex_normal.z );
    normal_map = normalize(vertex_basis * normal_map);

    //accumulate color
    vec3 color = ambient;
    for (int i = 0; i < int(NumLights.x) && i < MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        vec3 light_dir = normalize(light.pos.xyz);
        float light_dot_normal = dot(light_dir, normal_map);
        float diffuse = max(light_dot_normal, 0.0);
        diffuse = smoothstep(0.005, 0.01, diffuse);
        color += diffuse * light.color.xyz;
    }

    o_Target = vec4(albedo.rgb * color, albedo.w);
}
