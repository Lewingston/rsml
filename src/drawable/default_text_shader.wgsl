
// Vertex shader

struct TransformationUniform {
    matrix: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> transform: TransformationUniform;

struct CameraUniform {
    view_proj: mat4x4<f32>
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
};


struct InstanceInput {
    @location(0) sprite: vec4<f32>,
    @location(1) texture: vec4<u32>
};


@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    instance: InstanceInput
) -> VertexOutput {

    let x_arr : array<u32, 6> = array<u32, 6>(0, 1, 1, 0, 1, 0);
    let y_arr : array<u32, 6> = array<u32, 6>(0, 0, 1, 0, 1, 1);

    let x : u32 = x_arr[in_vertex_index];
    let y : u32 = y_arr[in_vertex_index];

    let pos_x = instance.sprite.x + f32(x) * instance.sprite.z;
    let pos_y = instance.sprite.y + f32(y) * instance.sprite.w;

    let tex_x : u32 = instance.texture.x;
    let tex_y : u32 = instance.texture.y;

    let tex_w : u32 = instance.texture.z;
    let tex_h : u32 = instance.texture.w;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * transform.matrix * vec4<f32>(pos_x, pos_y, 0.0, 1.0);

    out.tex_coords = vec2<f32>(f32(tex_x + (x * tex_w)), f32(tex_y + ((1 - y) * tex_h)));

    return out;
}


// Fragment shader

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let tex_size = textureDimensions(t_diffuse);

    let tex_x = in.tex_coords.x / f32(tex_size.x);
    let tex_y = in.tex_coords.y / f32(tex_size.y);

    var tex_cords = vec2<f32>(tex_x, tex_y);

    return textureSample(t_diffuse, s_diffuse, tex_cords);
}
