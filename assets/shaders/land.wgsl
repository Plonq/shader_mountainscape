#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import bevy_shader_utils::perlin_noise_2d

@group(1) @binding(0)
var<uniform> color: vec4<f32>;
@group(1) @binding(1)
var<uniform> offset: f32;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
    // The above import includes color but only if the mesh/model has vertex colors,
    // so we must add a new one (vec3 since alpha isn't used).
    @location(10) color: vec3<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;
    // Use perlin noise to make the mountains
    var height = perlinNoise2(vec2<f32>(position.x + offset, position.z + offset));
    // In addition to the noise, offset using a sine wave to get the slow rippling effect
    position.y += height + sin(position.x * position.z + globals.time) * 0.08;
    // Using height in only two of the components means when height is 0 it's not black
    out.color = vec3<f32>(0.2, height, height);

    // The following is from the default vertex shader
    out.world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.uv = vertex.uv;

    return out;
}

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
    @location(10) color: vec3<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
