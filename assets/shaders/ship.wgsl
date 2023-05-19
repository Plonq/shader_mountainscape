#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import bevy_shader_utils::perlin_noise_3d

struct ShipMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: ShipMaterial;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
//    @location(4) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));

//    out.world_position += vec4(out.world_normal, 1.0) * sin(globals.time) * 0.5;

    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.uv = vertex.uv;
//    out.color = vertex.color;

    return out;
}

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return material.color;

//    var noise = perlinNoise3(vec3(in.uv * 15.0, 1.0));
//    var noise = perlinNoise3(in.clip_position);
//    noise = (noise + 1.0) * 0.5;

//    return vec4<f32>(noise, noise, noise, 1.0);
}
