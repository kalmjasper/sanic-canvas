#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SafeColor {
    @align(16)
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> color: SafeColor;
@group(2) @binding(1)
var<uniform> outline_color: SafeColor;
@group(2) @binding(2)
var<uniform> outline_thickness: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Ensure we're drawing within the square bounds
    if (any(mesh.uv < vec2<f32>(0.0)) || any(mesh.uv > vec2<f32>(1.0))) {
        discard;
    }
    // Calculate distance from edge
    let distance_from_edge = min(
        min(mesh.uv.x, 1.0 - mesh.uv.x),
        min(mesh.uv.y, 1.0 - mesh.uv.y)
    );
    // If we're within the outline thickness, return outline color
    if (distance_from_edge < outline_thickness) {
        return outline_color.color;
    }

    return color.color;
}