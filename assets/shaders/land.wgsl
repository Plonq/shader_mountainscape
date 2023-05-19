#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import bevy_shader_utils::perlin_noise_2d

//struct LandMaterial {
//    color: vec4<f32>,
//    offset: f32,
//}

//@group(1) @binding(0)
//var<uniform> material: LandMaterial;
@group(1) @binding(0)
var<uniform> color: vec4<f32>;
@group(1) @binding(1)
var<uniform> offset: f32;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
//    @location(4) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
    @location(10) color: vec3<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.world_normal = mesh_normal_local_to_world(vertex.normal);

    var position = vertex.position;
    var height = perlinNoise2(vec2<f32>(position.x + offset, position.z + offset));
    var offset_noise = perlinNoise2(vec2<f32>(position.z, position.x));
    position.y += height + sin(position.x * position.z + globals.time) * 0.08;
    out.color = vec3<f32>(0.2, height, height);

    out.world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.uv = vertex.uv;
//    out.color = vertex.color;

    return out;
}

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
    @location(10) color: vec3<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
//    return vec4<f32>(in.uv, 0.0, 1.0);
    return vec4<f32>(in.color, 1.0);

//    var noise = perlinNoise3(vec3(in.uv * 15.0, 1.0));
//    var noise = perlinNoise3(in.clip_position);
//    noise = (noise + 1.0) * 0.5;

//    return vec4<f32>(noise, noise, noise, 1.0);
}
