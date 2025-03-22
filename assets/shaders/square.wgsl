#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Ensure we're drawing within the square bounds
    if (any(mesh.uv < vec2<f32>(0.0)) || any(mesh.uv > vec2<f32>(1.0))) {
        discard;
    }
    return color;
}