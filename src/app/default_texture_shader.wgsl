
// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color:    u32
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
}


@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {

    var out: VertexOutput;
    var color = model.color;

    let r : f32 = f32((color >> 24u) & 0xFFu) / 255.0;
    let g : f32 = f32((color >> 16u) & 0xFFu) / 255.0;
    let b : f32 = f32((color >> 8u ) & 0xFFu) / 255.0;
    let a : f32 = f32((color >> 0u ) & 0xFFu) / 255.0;

    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = vec4<f32>(r, g, b, a);

    return out;
}


// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    return vec4<f32>(in.color);
}
